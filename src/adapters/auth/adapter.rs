use crate::ports::auth_port::AuthPort;

pub struct JwtAdapter {
    base_url: String,
    client_id: String,
    client_secret: String,
}

impl JwtAdapter {
    pub fn new(base_url: String, client_id: String, client_secret: String) -> Self {
        Self {
            base_url,
            client_id,
            client_secret,
        }
    }
}

impl AuthPort for JwtAdapter {
    fn login(&self, _email: &str, _password: &str) -> Result<String, String> {
        todo!()
    }
}
