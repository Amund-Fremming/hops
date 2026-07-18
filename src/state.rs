use std::sync::Arc;
use std::time::Duration;

use sqlx::{Pool, Postgres};
use tracing::{error, info};

use crate::config::CONFIG;
use crate::db::otp::delete_expired_otps;
use crate::ports::comms::CommsPort;
use crate::ports::crypto::CryptoPort;
use crate::services::auth::AuthService;

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
    pub auth: Arc<AuthService>,
    pub comms: Arc<dyn CommsPort>,
    pub crypto: Arc<dyn CryptoPort>,
}

impl AppState {
    pub fn new(
        pool: Pool<Postgres>,
        auth: Arc<AuthService>,
        comms: Arc<dyn CommsPort>,
        crypto: Arc<dyn CryptoPort>,
    ) -> Self {
        Self {
            pool,
            auth,
            comms,
            crypto,
        }
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    /// Deletes expired OTP codes
    pub fn spawn_otp_cron_job(&self) {
        let cleanup_pool = self.pool.clone();
        let cleanup_interval = Duration::from_secs(CONFIG.otp.cleanup_interval_minutes as u64 * 60);

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(cleanup_interval).await;
                match delete_expired_otps(&cleanup_pool).await {
                    Ok(count) if count > 0 => info!("Cleaned up {} expired OTPs", count),
                    Err(e) => error!("OTP cleanup failed: {}", e),
                    _ => {}
                }
            }
        });
    }
}
