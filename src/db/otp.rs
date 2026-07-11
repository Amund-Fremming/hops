use std::time::Duration;

use chrono::Utc;
use sqlx::{Pool, Postgres};
use tracing::info;
use uuid::Uuid;

use crate::error::ServerError;
use crate::models::otp::{Otp, OtpResponse};

pub async fn get_otp_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Otp, ServerError> {
    todo!()
}

pub async fn get_otp_by_phone_number(
    pool: &Pool<Postgres>,
    phone_number: &str,
) -> Result<Otp, ServerError> {
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
        None => Err(ServerError::Otp("Otp entry does not exist".to_string())),
    }
}

/// TODO
/// - missing validation for
///     - max failed attempts
pub async fn verify_otp(
    pool: &Pool<Postgres>,
    phone_number: &str,
    code: &str,
) -> Result<(), ServerError> {
    let otp = get_otp_by_phone_number(pool, phone_number).await?;

    if otp.expires_at < Utc::now() {
        return Err(ServerError::Otp("Expired code".to_string()));
    }

    let hash = Otp::hash_code(code);
    if hash != otp.hash {
        return Err(ServerError::Otp("Invalid code".to_string()));
    }

    info!("Successfull verified OPT");
    Ok(())
}

/// TODO
/// - missing validation for
///     - max otp messages per day (24 hours)
///     - rate limit?
pub async fn create_otp(
    pool: &Pool<Postgres>,
    phone_number: &str,
) -> Result<OtpResponse, ServerError> {
    let code = Otp::generate_code();
    let hash = Otp::hash_code(&code);
    let expires_at = Utc::now() + Duration::from_mins(5);

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
