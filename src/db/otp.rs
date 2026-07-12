use std::time::Duration;

use chrono::Utc;
use sqlx::{Pool, Postgres};
use tracing::info;
use uuid::Uuid;

use crate::config::CONFIG;
use crate::models::otp::{Otp, OtpError, OtpResponse};

pub async fn get_otp_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Otp, OtpError> {
    todo!()
}

pub async fn get_otp_by_phone_number(
    pool: &Pool<Postgres>,
    phone_number: &str,
) -> Result<Otp, OtpError> {
    let otp = sqlx::query_as!(
        Otp,
        r#"
        SELECT id, phone_number, hash, expires_at, verified_at, created_at, ip_address, failed_attempts
        FROM "otp"
        WHERE phone_number = $1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        phone_number
    )
    .fetch_optional(pool)
    .await?;

    match otp {
        Some(otp) => Ok(otp),
        None => Err(OtpError::NotFound),
    }
}

pub async fn verify_otp(
    pool: &Pool<Postgres>,
    phone_number: &str,
    code: &str,
) -> Result<(), OtpError> {
    let otp = get_otp_by_phone_number(pool, phone_number).await?;

    if otp.failed_attempts >= CONFIG.otp.max_attempts as i32 {
        return Err(OtpError::MaxAttemptsExceeded);
    }

    if otp.expires_at < Utc::now() {
        return Err(OtpError::Expired);
    }

    let hash = Otp::hash_code(code);
    if hash != otp.hash {
        return Err(OtpError::WrongCode);
    }

    info!("Successfull verified OPT");
    Ok(())
}

pub async fn get_otp_count_today(
    pool: &Pool<Postgres>,
    phone_number: &str,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM "otp"
        WHERE phone_number = $1 AND created_at >= CURRENT_DATE
        "#,
        phone_number
    )
    .fetch_one(pool)
    .await
}

/// TODO - rate limit?
pub async fn create_otp(
    pool: &Pool<Postgres>,
    phone_number: &str,
) -> Result<OtpResponse, OtpError> {
    let code = Otp::generate_code();
    let hash = Otp::hash_code(&code);
    let expires_at = Utc::now() + Duration::from_mins(CONFIG.otp.ttl_minutes as u64);

    let otp_today = get_otp_count_today(pool, phone_number).await?;
    if otp_today >= i64::from(CONFIG.otp.max_messages_per_day) {
        return Err(OtpError::MaxMessagesExceeded);
    }

    let otp_id = sqlx::query_scalar!(
        r#"
        INSERT INTO "otp" (phone_number, hash, expires_at)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        phone_number,
        hash,
        expires_at
    )
    .fetch_one(pool)
    .await?;

    let response = OtpResponse { otp_id, code };
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn create_otp_successfull(pool: Pool<Postgres>) {
        let phone_number = "+4712345678";

        let result = create_otp(&pool, phone_number).await;

        assert!(result.is_ok());
        let otp_response = result.unwrap();
        assert_eq!(otp_response.code.len(), 6);
    }

    // should fail when max otp in a day is exceeded
    // should fail if wrong code is used
    // should fail with correct code if max tries has exceeded
}
