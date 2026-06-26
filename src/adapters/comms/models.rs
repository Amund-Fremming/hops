use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SendSmsRequest {
    pub to: String,
    pub from: String,
    pub message: String,
}

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
