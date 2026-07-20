pub trait CryptoPort: Send + Sync {
    fn hash(&self, value: &str) -> String;
    fn verify(&self, value: &str, expected_hash: &str) -> bool;
    fn hash_password(&self, password: &str) -> Result<String, String>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String>;
}
