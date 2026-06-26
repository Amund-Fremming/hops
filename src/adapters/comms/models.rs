use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SendSmsRequest {
    pub to: String,
    pub from: String,
    pub message: String,
}
