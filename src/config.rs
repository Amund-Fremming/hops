use base64::{Engine, engine::general_purpose::STANDARD};
use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, de};

fn deserialize_base64_to_pem<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let base64_str = String::deserialize(deserializer)?;
    let bytes = STANDARD
        .decode(&base64_str)
        .map_err(|e| de::Error::custom(format!("invalid base64: {}", e)))?;

    String::from_utf8(bytes).map_err(|e| de::Error::custom(format!("invalid UTF-8 in PEM: {}", e)))
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommsConfig {
    pub username: String,
    pub password: String,
    pub from: String,
    pub otp_message_template: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    #[serde(
        deserialize_with = "deserialize_base64_to_pem",
        rename = "private_key_base64"
    )]
    private_key_pem: String,
    #[serde(
        deserialize_with = "deserialize_base64_to_pem",
        rename = "public_key_base64"
    )]
    public_key_pem: String,
    pub audience: String,
    pub issuer: String,
}

impl AuthConfig {
    pub fn private_key_pem(&self) -> &str {
        &self.private_key_pem
    }

    pub fn public_key_pem(&self) -> &str {
        &self.public_key_pem
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct OtpConfig {
    pub ttl_minutes: u8,
    pub max_attempts: u8,
    pub max_messages_per_day: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CryptoConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub comms: CommsConfig,
    pub auth: AuthConfig,
    pub otp: OtpConfig,
    pub crypto: CryptoConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name(&format!("config/{}", env)))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}

pub static CONFIG: Lazy<AppConfig> =
    Lazy::new(|| AppConfig::load().expect("Failed to load config"));
