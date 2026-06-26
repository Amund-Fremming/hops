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

pub async fn send_sms(
    username: &str,
    password: &str,
    from: &str,
    to: &str,
    message: &str,
) -> Result<SendSmsResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://api.46elks.com/a1/sms")
        .basic_auth(username, Some(password))
        .form(&[("from", from), ("to", to), ("message", message)])
        .send()
        .await?
        .json::<SendSmsResponse>()
        .await?;

    Ok(response)
}
