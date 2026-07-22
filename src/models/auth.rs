use std::fmt;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use rsa::{RsaPublicKey, pkcs8::DecodePublicKey, traits::PublicKeyParts};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::CONFIG, error::ServerError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    access_token_expiry: i64,
    refresh_token_expiry: i64,
}

impl TokenResponse {
    pub fn new(
        user_id: Uuid,
        device_id: Uuid,
        access_token: String,
        refresh_token: String,
    ) -> Self {
        Self {
            user_id,
            device_id,
            access_token,
            refresh_token,
            access_token_expiry: CONFIG.auth.access_token_lifetime_minutes,
            refresh_token_expiry: CONFIG.auth.refresh_token_lifetime_days,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn user_id(&self) -> Result<Uuid, ServerError> {
        self.sub
            .parse::<Uuid>()
            .map_err(|e| ServerError::Auth(format!("Invalid user_id in token: {}", e)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub kty: String,
    #[serde(rename = "use")]
    pub _use: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

impl Jwk {
    pub fn new(kid: &str, public_key_pem: &str) -> Result<Self, rsa::pkcs8::spki::Error> {
        let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)?;
        Ok(Self {
            kty: "RSA".to_string(),
            _use: "sig".to_string(),
            alg: "RS256".to_string(),
            kid: kid.to_string(),
            n: URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be()),
            e: URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwks {
    pub keys: [Jwk; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHistory {
    pub user_id: Uuid,
    pub pwd_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserIdentity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: String,
    pub provider_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum ProviderType {
    Phone,
    Email,
    Social,
}

impl ProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Phone => "phone",
            Self::Email => "email",
            Self::Social => "social",
        }
    }
}

impl fmt::Display for ProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct LoginObject {
    pub user_id: Uuid,
    pub identity_id: Uuid,
    pub password_hash: String,
    pub is_locked: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserCredential {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub password_hash: String,
    pub failed_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub device_id: Uuid,
    pub device_name: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}
