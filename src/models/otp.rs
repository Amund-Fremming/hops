use chrono::{DateTime, Utc};
use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub struct OtpResponse {
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
