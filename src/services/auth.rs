use std::sync::Arc;
use std::time::Duration;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::{error, warn};
use uuid::Uuid;

use crate::{
    config::CONFIG,
    db::{
        self,
        audit::create_audit,
        auth::{get_phone_login_object, increment_failed_attempts, reset_failed_attempts},
        user::{create_user, is_phone_in_use},
    },
    error::ServerError,
    models::{
        audit::{AuditBuilder, ResourceType},
        auth::ProviderType,
    },
};
use crate::{
    db::{
        auth::{create_credential, create_identity},
        otp::get_otp_by_id,
    },
    models::{
        auth::{Claims, Jwk, Jwks, TokenResponse},
        user::User,
    },
    ports::crypto::CryptoPort,
};

pub struct AuthService {
    pool: Pool<Postgres>,
    crypto: Arc<dyn CryptoPort>,
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
        crypto: Arc<dyn CryptoPort>,
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
            crypto,
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

    pub async fn generate_tokens(&self, user_id: Uuid) -> Result<TokenResponse, ServerError> {
        let access_token_lifetime = CONFIG.auth.access_token_lifetime_minutes.clone();

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

        // TODO - insert refresh token to database

        let hash = self.crypto.hash(&refresh_token);
        let refresh_token_expiry =
            Utc::now() + Duration::from_hours(24 * CONFIG.auth.refresh_token_lifetime_days as u64);

        db::auth::create_refresh_token(
            &self.pool,
            user_id,
            &hash,
            refresh_token_expiry,
            None, // TODO - use this
            None, // TODO use device id
        )
        .await?;

        Ok(TokenResponse {
            access_token,
            refresh_token,
            expires_in: access_token_lifetime,
        })
    }

    /// TODO:
    /// - optimize 5/6 database trips
    pub async fn phone_signup(
        &self,
        otp_id: Uuid,
        given_name: &str,
        family_name: &str,
        password: &str,
    ) -> Result<(Uuid, TokenResponse), ServerError> {
        let otp = get_otp_by_id(&self.pool, otp_id).await?;

        if !otp.is_verified() {
            return Err(ServerError::Auth("Phone number not verified".to_string()));
        }

        let phone_number = otp.phone_number;

        if is_phone_in_use(&self.pool, &phone_number).await? {
            warn!(phone_number = %phone_number, "Signup attempted with phone number already in use");
            return Err(ServerError::Conflict);
        }

        let mut user = User::new(given_name, family_name);
        user.phone_number = Some(phone_number.clone());
        user.phone_number_verified = true;

        let mut tx = self.pool.begin().await?;
        create_user(&mut *tx, &user).await?;
        let identity =
            create_identity(&mut *tx, user.id, ProviderType::Phone, &phone_number).await?;

        let password_hash = self
            .crypto
            .hash_password(password)
            .map_err(|e| ServerError::Auth(e))?;
        create_credential(&mut *tx, identity.id, &password_hash).await?;
        tx.commit().await?;

        let token_response = self.generate_tokens(user.id).await?;
        Ok((user.id, token_response))
    }

    pub async fn phone_login(
        &self,
        phone_number: &str,
        password: &str,
    ) -> Result<TokenResponse, ServerError> {
        let max_attempts = CONFIG.auth.max_failed_login_attempts;
        let Some(login_object) =
            get_phone_login_object(&self.pool, phone_number, max_attempts).await?
        else {
            warn!(phone_number = %phone_number, "Login failed: could not find user with credentials");
            return Err(ServerError::NotFound);
        };

        if login_object.is_locked {
            warn!(phone_number = %phone_number, "Login failed: account locked");
            return Err(ServerError::AccountLocked);
        }

        if !self
            .crypto
            .verify_password(password, &login_object.password_hash)
            .map_err(|e| ServerError::Auth(e))?
        {
            warn!(phone_number = %phone_number, "Login failed: wrong password");
            increment_failed_attempts(&self.pool, login_object.identity_id).await?;
            return Err(ServerError::Auth("Login failed".to_string()));
        }

        reset_failed_attempts(&self.pool, login_object.identity_id).await?;

        let user_id = login_object.user_id;
        let phone_number = phone_number.to_string();
        let pool = self.pool.clone();

        tokio::task::spawn(async move {
            let log = AuditBuilder::new()
                .resource_id(user_id)
                .resource_type(ResourceType::User)
                .metadata(json!({
                    "phone_number": phone_number,
                }))
                .build();

            if let Err(e) = create_audit(&pool, &log).await {
                error!("Failed to create audit log on `phone_login`: {}", e);
            }
        });

        self.generate_tokens(login_object.user_id).await
    }

    pub async fn get_identities(&self, user_id: Uuid) -> Result<Vec<String>, ServerError> {
        /*
            Get all users identities, and then if they are currently logged in with what devices
            Create a pretty struct to retunr to user.
        */
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

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse, ServerError> {
        todo!()
    }
}
