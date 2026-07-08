use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneLoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneSignupRequest {
    pub phone: String,
    pub password: String,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub email: Option<String>,
    pub email_verified: bool,
    pub given_name: String,
    pub family_name: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
