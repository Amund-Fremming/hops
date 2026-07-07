use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    TokenExpired,
    RateLimited,
}

#[async_trait]
pub trait AuthPort: Send + Sync {
    fn verify_token(&self, token: &str, public_key: &str) -> Result<bool, AuthError>;

    fn get_decoding_key(&self, public_key: &str) -> Result<DecodingKey, AuthError>;

    fn get_encoding_key(&self, public_key: &str) -> Result<EncodingKey, AuthError>;

    async fn authenticate(&self, phone: &str, password: &str) -> Result<TokenResponse, AuthError>;

    async fn issue_token(&self, user_id: Uuid) -> Result<TokenResponse, AuthError>;

    async fn rotate_tokens(&self) -> Result<String, AuthError>;
}
