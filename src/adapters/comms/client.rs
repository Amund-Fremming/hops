use super::models::*;

pub struct CommsClient {
    base_url: String,
    api_key: String,
}

impl CommsClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self { base_url, api_key }
    }
}
