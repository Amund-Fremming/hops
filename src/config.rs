use config::{Config, ConfigError, File};
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name(&format!("config/{}", env)))
            .build()?;

        config.try_deserialize()
    }
}

pub static CONFIG: Lazy<AppConfig> =
    Lazy::new(|| AppConfig::load().expect("Failed to load config"));
