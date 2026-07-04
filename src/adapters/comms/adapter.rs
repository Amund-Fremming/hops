use async_trait::async_trait;

use crate::adapters::comms::models::SendSmsResponse;
use crate::ports::comms_port::CommsPort;

pub struct CommsAdapter {
    username: String,
    password: String,
}

impl CommsAdapter {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

#[async_trait]
impl CommsPort for CommsAdapter {
    async fn send_sms(
        &self,
        from: &str,
        to: &str,
        message: &str,
    ) -> Result<SendSmsResponse, reqwest::Error> {
        let client = reqwest::Client::new();

        let response = client
            .post("https://api.46elks.com/a1/sms")
            .basic_auth(&self.username, Some(&self.password))
            .form(&[("from", from), ("to", to), ("message", message)])
            .send()
            .await?
            .json::<SendSmsResponse>()
            .await?;

        Ok(response)
    }
}
