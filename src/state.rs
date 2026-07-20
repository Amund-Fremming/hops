use std::sync::Arc;
use std::time::Duration;

use sqlx::{Pool, Postgres};
use tracing::{error, info};
use uuid::Uuid;

use crate::config::CONFIG;
use crate::db::audit::delete_older_than;
use crate::db::otp::delete_expired_otps;
use crate::error::ServerError;
use crate::models::auth::Claims;
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

    pub fn require_admin(&self, claims: &Claims) -> Result<(), ServerError> {
        let user_id = claims
            .sub
            .parse::<Uuid>()
            .map_err(|_| ServerError::Forbidden)?;
        if !CONFIG.admin.is_admin(&user_id) {
            return Err(ServerError::Forbidden);
        }
        Ok(())
    }

    /// Deletes expired OTP codes
    pub fn spawn_otp_cron_job(&self) {
        info!("🧹 Starting OTP cron job");
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

    /// Deletes audit logs older than configured retention period
    pub fn spawn_audit_cron_job(&self) {
        info!("🧹 Starting audit log cron job ");
        let cleanup_pool = self.pool.clone();
        let cleanup_interval =
            Duration::from_secs(CONFIG.audit.cleanup_interval_hours as u64 * 3600);
        let retention_days = CONFIG.audit.retention_days;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(cleanup_interval).await;
                match delete_older_than(&cleanup_pool, retention_days).await {
                    Ok(count) if count > 0 => info!("Cleaned up {} old audit logs", count),
                    Err(e) => error!("Audit log cleanup failed: {}", e),
                    _ => {}
                }
            }
        });
    }
}
