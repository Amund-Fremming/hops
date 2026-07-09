use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::error::ServerError;
use crate::models::otp::{Otp, OtpResponse};

pub async fn get_otp_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Otp, ServerError> {
    todo!()
}

pub async fn create_otp(
    pool: &Pool<Postgres>,
    phone_number: &str,
) -> Result<OtpResponse, ServerError> {
    let code = Otp::generate_code();
    let hash = Otp::hash_code(&code);

    let otp_id = sqlx::query_scalar!(
        r#"
        INSERT INTO "otp" (phone_number, hash)
        VALUES ($1, $2)
        RETURNING id
        "#,
        phone_number,
        hash
    )
    .fetch_one(pool)
    .await?;

    let response = OtpResponse { otp_id, code };
    Ok(response)
}
