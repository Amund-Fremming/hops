use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::sync::Arc;
use uuid::Uuid;

use crate::ports::auth_port::{AuthError, AuthPort, TokenResponse};
use crate::ports::auth_repository::AuthRepository;

pub struct JwtAdapter {
    repo: Arc<dyn AuthRepository>,
}

impl JwtAdapter {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl AuthPort for JwtAdapter {
    fn verify_token(&self, token: &str, public_key: &str) -> Result<bool, AuthError> {
        todo!()
    }

    fn get_decoding_key(&self, public_key: &str) -> Result<DecodingKey, AuthError> {
        todo!()
    }

    fn get_encoding_key(&self, public_key: &str) -> Result<EncodingKey, AuthError> {
        todo!()
    }

    async fn authenticate(&self, phone: &str, password: &str) -> Result<TokenResponse, AuthError> {
        todo!()
    }

    async fn issue_token(&self, user_id: Uuid) -> Result<TokenResponse, AuthError> {
        todo!()
    }

    async fn rotate_tokens(&self) -> Result<String, AuthError> {
        todo!()
    }
}
