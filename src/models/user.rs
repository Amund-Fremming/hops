use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneLoginRequest {
    pub phone_number: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneSignupRequest {
    pub phone_number: String,
    pub password: String,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub phone_number: Option<String>,
    pub phone_number_verified: bool,
    pub email: Option<String>,
    pub email_verified: bool,
    pub given_name: String,
    pub family_name: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(given_name: &str, family_name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            given_name: given_name.to_string(),
            family_name: family_name.to_string(),
            phone_number: None,
            phone_number_verified: false,
            email: None,
            email_verified: false,
            avatar_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl From<PhoneSignupRequest> for User {
    fn from(req: PhoneSignupRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            phone_number: Some(req.phone_number),
            phone_number_verified: false,
            email: None,
            email_verified: false,
            given_name: req.given_name,
            family_name: req.family_name,
            avatar_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
