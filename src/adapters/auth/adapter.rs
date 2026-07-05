use async_trait::async_trait;

use crate::{domain::error::ServerError, ports::auth_port::AuthPort};

pub struct JwtAdapter {
    base_url: String,
    client_id: String,
    client_secret: String,
}

impl JwtAdapter {
    pub fn new(base_url: String, client_id: String, client_secret: String) -> Self {
        Self {
            base_url,
            client_id,
            client_secret,
        }
    }
}

#[async_trait]
impl AuthPort for JwtAdapter {
    async fn login(&self, phone: &str, password: &str) -> Result<String, ServerError> {
        todo!()
    }

    async fn rotate_tokens(&self) -> Result<String, ServerError> {
        todo!()
    }
}
