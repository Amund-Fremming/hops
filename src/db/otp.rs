use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::error::ServerError;
use crate::models::otp::Otp;

pub async fn get_otp_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Otp, ServerError> {
    todo!()
}
