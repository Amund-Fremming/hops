use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Otp {
    pub id: Uuid,
    pub phone_number: String,
    pub hash: String,
    pub expires_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub failed_attempts: i32,
}

impl Otp {
    pub fn is_verified(&self) -> bool {
        self.verified_at.is_some()
    }
}
