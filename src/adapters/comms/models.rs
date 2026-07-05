use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Otp {
    pub id: Uuid,
    pub phone: String,
    pub hash: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub failed_attempts: i32,
}

#[derive(Debug, Serialize)]
pub struct SendSmsRequest {
    pub to: String,
    pub from: String,
    pub message: String,
}

/// Response object from `Elks46`
#[derive(Debug, Deserialize)]
pub struct SendSmsResponse {
    pub status: String,
    pub direction: String,
    pub from: String,
    pub created: String,
    pub parts: u32,
    pub to: String,
    pub cost: u64,
    pub message: String,
    pub id: String,
}
