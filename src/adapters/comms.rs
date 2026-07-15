use async_trait::async_trait;

use crate::models::comms::{MakeCallResponse, SendSmsResponse};
use crate::ports::comms::CommsPort;

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

    async fn make_call(
        &self,
        from: &str,
        to: &str,
        audio_url: &str,
    ) -> Result<MakeCallResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let voice_start = format!(r#"{{"play":"{}"}}"#, audio_url);

        let response = client
            .post("https://api.46elks.com/a1/calls")
            .basic_auth(&self.username, Some(&self.password))
            .form(&[
                ("from", from),
                ("to", to),
                ("voice_start", voice_start.as_str()),
            ])
            .send()
            .await?
            .json::<MakeCallResponse>()
            .await?;

        Ok(response)
    }
}
