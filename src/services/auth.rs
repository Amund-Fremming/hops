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
        auth::{
            get_login_credentials, increment_failed_attempts, lock_account, reset_failed_attempts,
        },
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
                error!("Failed to create suspicious audit log: {}", e);
            }
        });
    }

    fn audit_account_locked(&self, user_id: Uuid, lock_hours: i64) {
        let pool = self.pool.clone();

        tokio::spawn(async move {
            let log = AuditBuilder::new()
                .resource_id(user_id)
                .resource_type(ResourceType::User)
                .action(Action::AccountLocked)
                .metadata(json!({ "lock_hours": lock_hours }))
                .build();

            if let Err(e) = create_audit(&pool, &log).await {
                error!("Failed to create account locked audit log: {}", e);
            }
        });
    }

    /// TODO:
    /// - optimize 5/6 database trips
    pub async fn signup(
        &self,
        otp_id: Uuid,
        provider_type: ProviderType,
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

        let identifier = otp.identifier;

        if is_phone_in_use(&self.pool, &identifier).await? {
            warn!(phone_number = %identifier, "Signup attempted with phone number already in use");
            return Err(ServerError::Conflict);
        }

        let mut user = User::new(given_name, family_name);
        match provider_type {
            ProviderType::Email => {
                user.email = Some(identifier.clone());
                user.email_verified = true;
            }
            ProviderType::Phone => {
                user.phone_number = Some(identifier.clone());
                user.phone_number_verified = true;
            }
        }

        let password_hash = self.crypto.hash_password(password)?;

        let mut tx = self.pool.begin().await?;
        db::user::create_user(&mut *tx, &user).await?;
        let identity =
            db::auth::create_identity(&mut *tx, user.id, ProviderType::Phone, &identifier).await?;
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

    pub async fn login(
        &self,
        device_id: Uuid,
        device_name: &str,
        user_agent: Option<&str>,
        identifier: &str, // email or phone_number
        provider_type: ProviderType,
        password: &str,
    ) -> Result<TokenResponse, ServerError> {
        let max_attempts = CONFIG.auth.max_failed_login_attempts;
        let lock_hours = CONFIG.auth.account_lock_hours;

        let Some(login_object) =
            get_login_credentials(&self.pool, identifier, provider_type).await?
        else {
            warn!(phone_number = %identifier, "Login failed: could not find user with credentials");
            return Err(ServerError::NotFound);
        };

        if login_object.is_locked() {
            warn!(phone_number = %identifier, "Login failed: account locked");
            return Err(ServerError::AccountLocked);
        }

        let is_valid = self
            .crypto
            .verify_password(password, &login_object.password_hash)?;

        if !is_valid {
            self.audit_suspicious(login_object.user_id, "Login failed: wrong password");
            warn!(phone_number = %identifier, "Login failed: wrong password");
            increment_failed_attempts(&self.pool, login_object.identity_id).await?;

            if login_object.failed_attempts + 1 >= max_attempts {
                lock_account(&self.pool, login_object.identity_id, lock_hours).await?;
                self.audit_account_locked(login_object.user_id, lock_hours);
                warn!(phone_number = %identifier, "Account locked due to max failed attempts");
            }

            return Err(ServerError::Auth("Login failed".to_string()));
        }

        reset_failed_attempts(&self.pool, login_object.identity_id).await?;

        let user_id = login_object.user_id;
        let phone_number = identifier.to_string();
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
    ## new (constructor)
    - new succeeds with valid RSA keys
    - new fails with invalid private key PEM
    - new fails with invalid public key PEM
    - new fails when Jwk construction fails

    ## get_jwks
    - get_jwks returns the JWKS with both keys
    - get_jwks returns keys with distinct kids

    ## validate_token
    - validate_token succeeds with valid token and returns claims
    - validate_token fails with expired token
    - validate_token fails with invalid signature
    - validate_token fails with wrong audience
    - validate_token fails with wrong issuer
    - validate_token rejects malformed tokens
    - validate_token rejects alg=none and HS256-signed tokens (algorithm confusion)
    - validate_token rejects tokens missing sub

    ## signup
    - signup succeeds with phone provider and returns token response
    - signup succeeds with email provider and returns token response
    - signup fails when OTP not found
    - signup fails when OTP is not verified
    - signup fails when identifier already in use (phone)
    - signup fails when identifier already in use (email)
    - signup sets phone_number and phone_number_verified for phone provider
    - signup sets email and email_verified for email provider
    - signup creates identity with the requested provider type (not always Phone)
    - signup rolls back user row when credential creation fails
    - signup rejects reuse of an already-consumed otp_id
    - signup persists session with device_name/user_agent and a hash matching the returned RT
    - signup succeeds with user_agent = None

    ## login
    - login succeeds with phone provider and creates new session
    - login succeeds with email provider and creates new session
    - login succeeds and updates existing session for known device
    - login fails when identifier not found (phone)
    - login fails when identifier not found (email)
    - login fails when account is locked
    - login fails when password is invalid
    - login locks account after max failed attempts reached
    - login increments failed_attempts without locking below max
    - login succeeds when locked_until has expired
    - login resets failed_attempts after success following failures
    - login propagates crypto verify_password errors
    - login writes a success audit log
    - login fails when identifier does not match provider_type
    - login succeeds with user_agent = None

    ## set_password
    - set_password succeeds with valid old password
    - set_password fails when credential not found for provider
    - set_password fails when old password equals new password
    - set_password fails when old password is invalid
    - set_password: new password verifies and old password is rejected afterwards
    - set_password propagates crypto verify errors

    ## refresh_token
    - refresh_token succeeds and returns new token pair
    - refresh_token fails when session not found for device
    - refresh_token fails and expires session when token is invalid
    - refresh_token updates session with new token hash after rotation
    - refresh_token fails when session is expired
    - refresh_token fails when session is revoked
    - refresh_token rejects a rotated (replayed) token and expires the session

    ## tokens
    - access token exp equals now + access_token_lifetime_minutes * 60
    - refresh token expiry equals now + refresh_token_lifetime_days
    - generate_refresh_token produces distinct 32-byte URL-safe values
    - token_response lifetimes agree with the values used to mint the tokens
    */
}
