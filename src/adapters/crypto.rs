use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::ports::crypto::CryptoPort;

type HmacSha256 = Hmac<Sha256>;

pub struct CryptoAdapter {
    secret: String,
}

impl CryptoAdapter {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl CryptoPort for CryptoAdapter {
    fn hash(&self, value: &str) -> String {
        let mut mac =
            HmacSha256::new_from_slice(self.secret.as_bytes()).expect("HMAC can take any size key");
        mac.update(value.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    fn verify(&self, value: &str, expected_hash: &str) -> bool {
        let hash = self.hash(value);
        hash == expected_hash
    }
}
