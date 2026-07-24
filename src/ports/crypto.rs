#[cfg(test)]
use mockall::automock;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Failed to hash password: {0}")]
    HashFailed(String),
    #[error("Invalid password hash: {0}")]
    InvalidHash(String),
}

#[cfg_attr(test, automock)]
pub trait CryptoPort: Send + Sync {
    fn hash(&self, value: &str) -> String;
    fn verify(&self, value: &str, expected_hash: &str) -> bool;
    fn hash_password(&self, password: &str) -> Result<String, CryptoError>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, CryptoError>;
}
