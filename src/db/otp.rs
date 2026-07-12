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

/// TODO
/// - missing validation for
///     - max failed attempts
pub async fn verify_otp(
    pool: &Pool<Postgres>,
    phone_number: &str,
    code: &str,
) -> Result<(), OtpError> {
    let otp = get_otp_by_phone_number(pool, phone_number).await?;

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

/// TODO
/// - missing validation for
///     - rate limit?
pub async fn create_otp(
    pool: &Pool<Postgres>,
    phone_number: &str,
) -> Result<OtpResponse, OtpError> {
    let code = Otp::generate_code();
    let hash = Otp::hash_code(&code);
    let expires_at = Utc::now() + Duration::from_mins(CONFIG.otp.ttl_minutes as u64);

    let otp_today = get_otp_count_today(pool, phone_number).await?;
    if otp_today >= CONFIG.otp.max_messages_per_day as i64 {
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
    async fn create_otp_successful(pool: Pool<Postgres>) {
        let phone_number = "+4712345678";

        let result = create_otp(&pool, phone_number).await;

        assert!(result.is_ok());
        let otp_response = result.unwrap();
        assert_eq!(otp_response.code.len(), 6);
    }

    #[sqlx::test]
    async fn create_otp_max_messages_exceeded(pool: Pool<Postgres>) {
        let phone_number = "+4712345678";
        let max_messages = CONFIG.otp.max_messages_per_day;

        for _ in 0..max_messages {
            create_otp(&pool, phone_number).await.unwrap();
        }

        let result = create_otp(&pool, phone_number).await;

        assert!(matches!(result, Err(OtpError::MaxMessagesExceeded)));
    }

    #[sqlx::test]
    async fn get_otp_by_phone_number_found(pool: Pool<Postgres>) {
        let phone_number = "+4712345678";
        create_otp(&pool, phone_number).await.unwrap();

        let result = get_otp_by_phone_number(&pool, phone_number).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().phone_number, phone_number);
    }

    #[sqlx::test]
    async fn get_otp_by_phone_number_not_found(pool: Pool<Postgres>) {
        let result = get_otp_by_phone_number(&pool, "+4799999999").await;

        assert!(matches!(result, Err(OtpError::NotFound)));
    }

    #[sqlx::test]
    async fn verify_otp_successful(pool: Pool<Postgres>) {
        let phone_number = "+4712345678";
        let otp_response = create_otp(&pool, phone_number).await.unwrap();

        let result = verify_otp(&pool, phone_number, &otp_response.code).await;

        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn verify_otp_wrong_code(pool: Pool<Postgres>) {
        let phone_number = "+4712345678";
        create_otp(&pool, phone_number).await.unwrap();

        let result = verify_otp(&pool, phone_number, "000000").await;

        assert!(matches!(result, Err(OtpError::WrongCode)));
    }

    #[sqlx::test]
    async fn verify_otp_expired(pool: Pool<Postgres>) {
        let phone_number = "+4712345678";
        let otp_response = create_otp(&pool, phone_number).await.unwrap();

        sqlx::query!(
            r#"UPDATE "otp" SET expires_at = $1 WHERE id = $2"#,
            Utc::now() - Duration::from_secs(60),
            otp_response.otp_id
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = verify_otp(&pool, phone_number, &otp_response.code).await;

        assert!(matches!(result, Err(OtpError::Expired)));
    }

    #[sqlx::test]
    async fn get_otp_count_today_returns_count(pool: Pool<Postgres>) {
        let phone_number = "+4712345678";

        let count_before = get_otp_count_today(&pool, phone_number).await.unwrap();
        assert_eq!(count_before, 0);

        create_otp(&pool, phone_number).await.unwrap();
        create_otp(&pool, phone_number).await.unwrap();

        let count_after = get_otp_count_today(&pool, phone_number).await.unwrap();
        assert_eq!(count_after, 2);
    }
}
