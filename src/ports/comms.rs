use async_trait::async_trait;

use crate::models::comms::{MakeCallResponse, SendSmsResponse};

#[async_trait]
pub trait CommsPort: Send + Sync {
    async fn send_sms(
        &self,
        from: &str,
        to: &str,
        message: &str,
    ) -> Result<SendSmsResponse, reqwest::Error>;

    async fn make_call(
        &self,
        from: &str,
        to: &str,
        audio_url: &str,
    ) -> Result<MakeCallResponse, reqwest::Error>;
}
