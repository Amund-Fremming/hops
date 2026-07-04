pub trait AuthPort {
    fn login(&self, email: &str, password: &str) -> Result<String, String>;
}
