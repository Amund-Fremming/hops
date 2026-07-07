use async_trait::async_trait;

use crate::adapters::comms::models::SendSmsResponse;

#[async_trait]
pub trait CommsPort: Send + Sync {
    async fn send_sms(
        &self,
        from: &str,
        to: &str,
        message: &str,
    ) -> Result<SendSmsResponse, reqwest::Error>;
}
