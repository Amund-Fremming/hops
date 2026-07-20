use std::time::Duration;

use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::otp::{Otp, OtpError, OtpResponse};

pub async fn get_otp_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Otp, OtpError> {
    let otp = sqlx::query_as!(
        Otp,
        r#"
        SELECT id, phone_number, hash, expires_at, verified_at, created_at, ip_address, failed_attempts
        FROM "otp"
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    match otp {
        Some(otp) => Ok(otp),
        None => Err(OtpError::NotFound),
    }
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

pub async fn increment_failed_attempts(
    pool: &Pool<Postgres>,
    otp_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE "otp"
        SET failed_attempts = failed_attempts + 1
        WHERE id = $1
        "#,
        otp_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn mark_verified(pool: &Pool<Postgres>, otp_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE "otp"
        SET verified_at = NOW()
        WHERE id = $1
        "#,
        otp_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_expired_otps(pool: &Pool<Postgres>) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM "otp"
        WHERE expires_at < NOW()
        "#
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

async fn get_otp_count_today(
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

pub async fn create_otp(
    pool: &Pool<Postgres>,
    phone_number: &str,
    hash: &str,
    ttl_minutes: u8,
    max_messages_per_day: i64,
) -> Result<OtpResponse, OtpError> {
    let expires_at = Utc::now() + Duration::from_mins(ttl_minutes as u64);

    let otp_today = get_otp_count_today(pool, phone_number).await?;
    if otp_today >= max_messages_per_day {
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

    let response = OtpResponse { otp_id };
    Ok(response)
}

pub async fn delete_otp(pool: &Pool<Postgres>, otp_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM "otp"
        WHERE id = $1
        "#,
        otp_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
