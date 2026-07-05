use async_trait::async_trait;

use crate::domain::error::ServerError;

#[async_trait]
pub trait AuthPort {
    async fn login(&self, phone: &str, password: &str) -> Result<String, ServerError>;
    async fn rotate_tokens(&self) -> Result<String, ServerError>;
}
