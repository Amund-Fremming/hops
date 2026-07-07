use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use rsa::{RsaPublicKey, pkcs8::DecodePublicKey, traits::PublicKeyParts};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwk {
    /// Key identifier
    pub kid: String,
    /// Key type
    pub kty: String,
    /// Key set usage
    #[serde(rename = "use")]
    pub _use: String,
    pub alg: String,
    /// Public key modulus (Base64URL)
    pub n: String,
    /// Public key exponent (Base64URL)
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
pub struct UserIdentity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: String,
    pub provider_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredential {
    pub identity_id: Uuid,
    pub pwd_hash: String,
    pub algorithm: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub failed_attempts: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub user_agent: Option<String>,
    pub device_id: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHistory {
    pub user_id: Uuid,
    pub pwd_hash: String,
    pub created_at: DateTime<Utc>,
}
