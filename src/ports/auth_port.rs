use async_trait::async_trait;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{adapters::auth::models::Jwks, domain::error::ServerError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

#[async_trait]
pub trait AuthPort: Send + Sync {
    fn get_jwks(&self) -> &Jwks;

    fn validate_token(&self, token: &str) -> Result<Claims, StatusCode>;

    fn generate_tokens(&self, user_id: Uuid) -> Result<TokenResponse, ServerError>;

    async fn authenticate(&self, phone: &str, password: &str)
    -> Result<TokenResponse, ServerError>;

    async fn get_identities(&self, user_id: Uuid) -> Result<Vec<String>, ServerError>;

    async fn set_password(
        &self,
        user_id: Uuid,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), ServerError>;

    async fn rotate_tokens(&self, refresh_token: &str) -> Result<TokenResponse, ServerError>;

    async fn increment_failed_attempts(&self, user_id: Uuid) -> Result<(), ServerError>;

    async fn reset_failed_attempts(&self, user_id: Uuid) -> Result<(), ServerError>;
}
