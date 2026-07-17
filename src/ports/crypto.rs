pub trait CryptoPort: Send + Sync {
    fn hash(&self, value: &str) -> String;
    fn verify(&self, value: &str, expected_hash: &str) -> bool;
}
