use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use reqwest::StatusCode;
use sqlx::{Pool, Postgres};
use tracing::warn;
use uuid::Uuid;

use crate::{
    db::auth::{create_credential, create_identity},
    models::{
        auth::{Claims, Jwk, Jwks, TokenResponse},
        user::User,
    },
};
use crate::{
    db::{auth::get_credential_by_phone, users::create_user},
    error::ServerError,
};

pub struct AuthService {
    pool: Pool<Postgres>,
    jwks: Jwks,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    audience: String,
    issuer: String,
}

impl AuthService {
    pub fn new(
        pool: Pool<Postgres>,
        private_key_pem: &str,
        public_key_pem: &str,
        audience: &str,
        issuer: &str,
    ) -> Result<Self, ServerError> {
        let encoding_key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
            .map_err(|e| ServerError::Auth(format!("Invalid private key: {}", e)))?;

        let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
            .map_err(|e| ServerError::Auth(format!("Invalid public key: {}", e)))?;

        let jwk1 = Jwk::new("key-1", public_key_pem)?;
        let jwk2 = Jwk::new("key-2", public_key_pem)?;
        let jwks = Jwks { keys: [jwk1, jwk2] };

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[audience]);
        validation.set_issuer(&[issuer]);

        Ok(Self {
            pool,
            jwks,
            encoding_key,
            decoding_key,
            validation,
            audience: audience.to_string(),
            issuer: issuer.to_string(),
        })
    }

    pub fn get_jwks(&self) -> &Jwks {
        &self.jwks
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, StatusCode> {
        let token_data =
            decode::<Claims>(token, &self.decoding_key, &self.validation).map_err(|e| {
                warn!("Token validation failed: {}", e);
                StatusCode::UNAUTHORIZED
            })?;

        let claims = token_data.claims;
        Ok(claims)
    }

    pub fn generate_tokens(&self, user_id: Uuid) -> Result<TokenResponse, ServerError> {
        let access_token_lifetime = 16 * 60; // 15 mins

        let claims = Claims {
            sub: user_id.to_string(),
            iss: self.issuer.clone(),
            aud: vec![self.audience.clone()],
            exp: (Utc::now().timestamp() + access_token_lifetime) as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let header = Header::new(Algorithm::RS256);
        let access_token = encode(&header, &claims, &self.encoding_key)
            .map_err(|e| ServerError::Auth(format!("Failed to encode AT: {:?}", e)))?;

        let refresh_token = {
            let bytes: [u8; 32] = rand::random();
            URL_SAFE_NO_PAD.encode(bytes)
        };

        Ok(TokenResponse {
            access_token,
            refresh_token,
            expires_in: access_token_lifetime,
        })
    }

    fn hash_password(password: &str) -> Result<String, ServerError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| ServerError::Auth(format!("Failed to hash password: {e}")))
    }

    fn verify_password(password: &str, hash: &str) -> Result<bool, ServerError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| ServerError::Auth(format!("Invalid password hash: {e}")))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub async fn phone_signup(
        &self,
        given_name: &str,
        family_name: &str,
        phone: &str,
        password: &str,
    ) -> Result<(Uuid, TokenResponse), ServerError> {
        let password_hash = Self::hash_password(password)?;

        // TODO - validate that phone is verified in the otp table

        let mut user = User::new(given_name, family_name);
        user.phone = Some(phone.to_string());
        user.phone_verified = true;

        // TODO optimize 5 db trips

        let mut tx = self.pool.begin().await?;
        create_user(&mut *tx, &user).await?;
        create_identity(&mut *tx, user.id, "phone", phone).await?;
        create_credential(&mut *tx, user.id, &password_hash).await?;
        tx.commit().await?;

        let token_response = self.generate_tokens(user.id)?;
        Ok((user.id, token_response))
    }

    pub async fn phone_login(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<TokenResponse, ServerError> {
        let Some(credentials) = get_credential_by_phone(&self.pool, phone).await? else {
            warn!(phone = %phone, "Login failed: could not find phone identity for user.");
            return Err(ServerError::NotFound);
        };

        let pasword_hash = Self::hash_password(password)?;
        if pasword_hash != credentials.password_hash {
            warn!(phone = %phone, "Login failed: wrong password.");
            return Err(ServerError::Auth("Login failed".to_string()));
        }

        self.generate_tokens(user_id)
    }

    pub async fn get_identities(&self, user_id: Uuid) -> Result<Vec<String>, ServerError> {
        todo!()
    }

    pub async fn set_password(
        &self,
        user_id: Uuid,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), ServerError> {
        todo!()
    }

    pub async fn rotate_tokens(&self, refresh_token: &str) -> Result<TokenResponse, ServerError> {
        todo!()
    }

    pub async fn increment_failed_attempts(&self, user_id: Uuid) -> Result<(), ServerError> {
        todo!()
    }

    pub async fn reset_failed_attempts(&self, user_id: Uuid) -> Result<(), ServerError> {
        todo!()
    }
}
