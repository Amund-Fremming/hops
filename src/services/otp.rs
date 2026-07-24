use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    config::{CommsConfig, OtpConfig},
    db,
    models::{
        auth::ProviderType,
        otp::{Otp, OtpError, OtpResponse},
    },
    ports::{comms::CommsPort, crypto::CryptoPort},
};

pub struct OtpService {
    config: OtpConfig,
    comms_config: CommsConfig,
    pool: Pool<Postgres>,
    comms: Arc<dyn CommsPort>,
    crypto: Arc<dyn CryptoPort>,
}

impl OtpService {
    pub fn new(
        config: OtpConfig,
        comms_config: CommsConfig,
        pool: Pool<Postgres>,
        comms: Arc<dyn CommsPort>,
        crypto: Arc<dyn CryptoPort>,
    ) -> Self {
        Self {
            config,
            comms_config,
            pool,
            comms,
            crypto,
        }
    }

    pub async fn create_and_send(&self, phone_number: &str) -> Result<OtpResponse, OtpError> {
        let code = Otp::generate_code();
        let hash = self.crypto.hash(&code);

        let response = db::otp::create_otp(
            &self.pool,
            phone_number,
            ProviderType::Phone,
            &hash,
            self.config.ttl_minutes,
            self.config.max_messages_per_day,
        )
        .await?;

        let message = self
            .comms_config
            .otp_message_template
            .replace("{code}", &code);

        if let Err(e) = self
            .comms
            .send_sms(&self.comms_config.from, phone_number, &message)
            .await
        {
            error!(
                otp_id = %response.otp_id,
                phone_number = %phone_number,
                error = %e,
                "Failed to send OTP, deleting entry"
            );
            db::otp::delete_otp(&self.pool, response.otp_id).await?;

            return Err(OtpError::SmsFailed);
        }

        info!(phone_number = %phone_number, "Created OTP entry");

        Ok(response)
    }

    pub async fn verify(&self, otp_id: Uuid, code: &str) -> Result<(), OtpError> {
        let otp = db::otp::get_otp_by_id(&self.pool, otp_id).await?;

        if otp.is_expired() {
            return Err(OtpError::Expired);
        }

        if otp.is_max_attempts_exceeded(self.config.max_attempts as i32) {
            return Err(OtpError::MaxAttemptsExceeded);
        }

        let valid_code = self.crypto.verify(code, &otp.hash);

        if !valid_code {
            warn!(
                otp_id = %otp_id,
                code = %code,
                phone_number = %otp.identifier,
                "Invalid code for OTP"
            );

            let pool = self.pool.clone();
            tokio::spawn(async move {
                if let Err(e) = db::otp::increment_failed_attempts(&pool, otp_id).await {
                    error!(
                        otp_id = %otp_id,
                        error = %e,
                        "Failed to increment failed OTP attempts"
                    );
                }
            });

            return Err(OtpError::WrongCode);
        }

        db::otp::mark_verified(&self.pool, otp_id).await?;

        Ok(())
    }
}
