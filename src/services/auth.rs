use std::sync::Arc;
use std::time::Duration;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    config::{AuthConfig, CONFIG},
    db::{
        self,
        audit::create_audit,
        auth::{get_phone_login_object, increment_failed_attempts, reset_failed_attempts},
        otp::get_otp_by_id,
        user::is_phone_in_use,
    },
    error::ServerError,
    models::{
        audit::{Action, AuditBuilder, ResourceType},
        auth::{Claims, Jwk, Jwks, ProviderType, TokenResponse},
        user::User,
    },
    ports::crypto::CryptoPort,
};

pub struct AuthService {
    config: AuthConfig,
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
        config: AuthConfig,
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
            config,
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

    fn generate_access_token(&self, user_id: Uuid) -> Result<String, ServerError> {
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

        Ok(access_token)
    }

    /// Refresh token + hash
    fn generate_refresh_token(&self) -> String {
        let refresh_token = {
            let bytes: [u8; 32] = rand::random();
            URL_SAFE_NO_PAD.encode(bytes)
        };
        refresh_token
    }

    fn audit_suspicious(&self, user_id: Uuid, description: &str) {
        let pool = self.pool.clone();
        let description = description.to_string();

        tokio::spawn(async move {
            let log = AuditBuilder::new()
                .resource_id(user_id)
                .resource_type(ResourceType::User)
                .action(Action::Suspicious)
                .metadata(json!({ "description": description }))
                .build();

            if let Err(e) = create_audit(&pool, &log).await {
                error!(
                    error = %e,
                    "Failed to create suspicious audit log"
                );
            }
        });
    }

    /// TODO:
    /// - optimize 5/6 database trips
    pub async fn phone_signup(
        &self,
        otp_id: Uuid,
        device_name: &str,
        user_agent: Option<&str>,
        given_name: &str,
        family_name: &str,
        password: &str,
    ) -> Result<TokenResponse, ServerError> {
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

        let password_hash = self.crypto.hash_password(password)?;

        let mut tx = self.pool.begin().await?;
        db::user::create_user(&mut *tx, &user).await?;
        let identity =
            db::auth::create_identity(&mut *tx, user.id, ProviderType::Phone, &phone_number)
                .await?;
        db::auth::create_credential(&mut *tx, identity.id, &password_hash).await?;
        tx.commit().await?;

        let device_id = Uuid::new_v4();
        let at = self.generate_access_token(user.id)?;
        let rt = self.generate_refresh_token();
        let rt_hash = self.crypto.hash(&rt);
        let rt_expiry = Self::refresh_token_expiry();

        db::auth::create_session(
            &self.pool,
            user.id,
            device_id,
            device_name,
            &rt_hash,
            rt_expiry,
            user_agent,
        )
        .await?;

        Ok(self.token_response(at, rt))
    }

    fn refresh_token_expiry() -> DateTime<Utc> {
        Utc::now() + Duration::from_hours(24 * CONFIG.auth.refresh_token_lifetime_days as u64)
    }

    pub async fn phone_login(
        &self,
        device_id: Uuid,
        device_name: &str,
        user_agent: Option<&str>,
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

        let is_valid = self
            .crypto
            .verify_password(password, &login_object.password_hash)?;

        if !is_valid {
            self.audit_suspicious(login_object.user_id, "Login failed: wrong password");
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

        let at = self.generate_access_token(user_id)?;
        let rt = self.generate_refresh_token();
        let rt_hash = self.crypto.hash(&rt);
        let rt_expiry = Self::refresh_token_expiry();

        db::auth::upsert_session(
            &self.pool,
            user_id,
            device_id,
            device_name,
            &rt_hash,
            rt_expiry,
            user_agent,
        )
        .await?;

        Ok(self.token_response(at, rt))
    }

    pub async fn set_password(
        &self,
        user_id: Uuid,
        provider_type: ProviderType,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), ServerError> {
        let Some(user_credential) =
            db::auth::get_credential(&self.pool, user_id, &provider_type).await?
        else {
            self.audit_suspicious(user_id, "Tried setting password on non-existent provider");
            warn!(
                user_id = %user_id,
                provider_type = %provider_type,
                "User tried setting password on non existent provider type"
            );
            return Err(ServerError::Forbidden);
        };

        if old_password == new_password {
            return Err(ServerError::Auth(
                "New password must differ from current".to_string(),
            ));
        }

        let valid_old_password = self
            .crypto
            .verify_password(old_password, &user_credential.password_hash)?;

        if !valid_old_password {
            self.audit_suspicious(user_id, "Tried setting password with invalid old password");
            warn!(
                user_id = %user_id,
                provider_type = %provider_type,
                "User tried setting new password with invalid old password"
            );
            return Err(ServerError::Forbidden);
        }

        let new_password_hash = self.crypto.hash_password(new_password)?;
        db::auth::set_credential_password(&self.pool, user_credential.id, &new_password_hash)
            .await?;

        info!(
            user_id = %user_id,
            "User updated their password"
        );

        Ok(())
    }

    pub async fn refresh_token(
        &self,
        device_id: Uuid,
        refresh_token: &str,
    ) -> Result<TokenResponse, ServerError> {
        let Some(session) = db::auth::get_session(&self.pool, device_id).await? else {
            warn!(
                device_id = %device_id,
                "Requested refresh token does not exist"
            );
            return Err(ServerError::Forbidden);
        };

        let user_id = session.user_id;
        let valid_token = self
            .crypto
            .verify(&refresh_token, &session.refresh_token_hash);

        if !valid_token {
            self.audit_suspicious(user_id, "Invalid refresh token attempt");
            warn!(
                session_id = %session.id,
                device_id = %device_id,
                "Invalid refresh token, invalidating session"
            );
            db::auth::expire_session(&self.pool, session.id).await?;
            return Err(ServerError::Forbidden);
        }

        let at = self.generate_access_token(user_id)?;
        let rt = self.generate_refresh_token();
        let rt_hash = self.crypto.hash(&rt);
        let rt_expiry = Self::refresh_token_expiry();
        let session_id = session.id;
        let pool = self.pool.clone();

        tokio::spawn(async move {
            if let Err(e) = db::auth::update_session(&pool, session_id, &rt_hash, rt_expiry).await {
                error!(
                    user_id = %user_id,
                    session_id = %session_id,
                    error = %e,
                    "Failed to update session with new refresh token"
                );
            };
        });

        Ok(self.token_response(at, rt))
    }

    fn token_response(&self, access_token: String, refresh_token: String) -> TokenResponse {
        TokenResponse {
            access_token,
            refresh_token,
            access_expires_in: self.config.access_token_lifetime_minutes,
            refresh_expires_in: self.config.refresh_token_lifetime_days * 24 * 60,
        }
    }
}

#[cfg(test)]
mod test {
    /*
    Add tests for
    - phone login success create new session when device id not present
    - phone login success updates session when device id not present
    - phone login fails when password is invalid
    - phone login fails when password is correct but the attempts have exceeded max
    - phone login fails when the account is locked

    - signup successfull
    - signup fails when no otp present
    - signup fails when otp not valid
    - signup fails when otp is expired
    - signup fails when phone number is in use

    - set password

    - refresh token
    */
}
