use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum OtpError {
    #[error("SMS failed to send")]
    SmsFailed,

    #[error("OTP expired")]
    Expired,

    #[error("Max attempts exceeded")]
    MaxAttemptsExceeded,

    #[error("Wrong code")]
    WrongCode,

    #[error("OTP not found")]
    NotFound,

    #[error("Max messages per day exceeded")]
    MaxMessagesExceeded,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Serialize)]
pub struct OtpResponse {
    pub otp_id: Uuid,
}

// TODO - add verification
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOtpRequest {
    pub phone_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyOtpRequest {
    pub otp_id: Uuid,
    pub code: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Otp {
    pub id: Uuid,
    pub phone_number: String,
    pub hash: String,
    pub expires_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub failed_attempts: i32,
}

impl Otp {
    pub fn is_verified(&self) -> bool {
        self.verified_at.is_some()
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    pub fn is_max_attempts_exceeded(&self, max: i32) -> bool {
        self.failed_attempts >= max
    }

    pub fn generate_code() -> String {
        let code: u32 = rand::thread_rng().gen_range(0..1_000_000);
        format!("{:06}", code)
    }

    pub fn hash_code(code: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(code.as_bytes());
        hex::encode(hasher.finalize())
    }
}
