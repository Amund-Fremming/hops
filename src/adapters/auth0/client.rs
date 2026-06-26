use super::models::*;

pub struct AuthClient {
    base_url: String,
    client_id: String,
    client_secret: String,
}

impl AuthClient {
    pub fn new(base_url: String, client_id: String, client_secret: String) -> Self {
        Self {
            base_url,
            client_id,
            client_secret,
        }
    }
}
