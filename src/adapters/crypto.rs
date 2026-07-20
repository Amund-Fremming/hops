use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
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

    fn hash_password(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| format!("Failed to hash password: {e}"))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| format!("Invalid password hash: {e}"))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
