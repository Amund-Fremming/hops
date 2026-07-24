use async_trait::async_trait;

use crate::models::comms::{SendEmailResponse, SendSmsResponse};

#[async_trait]
pub trait CommsPort: Send + Sync {
    async fn send_sms(
        &self,
        from: &str,
        to: &str,
        message: &str,
    ) -> Result<SendSmsResponse, reqwest::Error>;

    async fn send_email(
        &self,
        from: &str,
        to: &[&str],
        subject: &str,
        html: Option<&str>,
        text: Option<&str>,
    ) -> Result<SendEmailResponse, reqwest::Error>;
}
