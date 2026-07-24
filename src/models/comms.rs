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

#[derive(Debug, Serialize)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendEmailResponse {
    pub id: String,
}
