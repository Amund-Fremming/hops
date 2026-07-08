use chrono::{DateTime, Utc};
use sqlx::{Executor, Pool, Postgres};
use uuid::Uuid;

use crate::error::ServerError;
use crate::models::auth::{RefreshToken, UserCredential, UserIdentity};

pub async fn create_identity<'e, E>(
    exec: E,
    user_id: Uuid,
    provider_type: &str,
    provider_id: &str,
) -> Result<UserIdentity, ServerError>
where
    E: Executor<'e, Database = Postgres>,
{
    let identity = sqlx::query_as!(
        UserIdentity,
        r#"
        INSERT INTO user_identity (id, user_id, provider_type, provider_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, provider_type, provider_id, created_at
        "#,
        Uuid::new_v4(),
        user_id,
        provider_type,
        provider_id
    )
    .fetch_one(exec)
    .await?;

    Ok(identity)
}

pub async fn create_credential<'e, E>(
    exec: E,
    identity_id: Uuid,
    password_hash: &str,
) -> Result<UserCredential, ServerError>
where
    E: Executor<'e, Database = Postgres>,
{
    let credential = sqlx::query_as!(
        UserCredential,
        r#"
        INSERT INTO user_credential (id, identity_id, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, identity_id, password_hash, failed_attempts, locked_until, created_at, updated_at
        "#,
        Uuid::new_v4(),
        identity_id,
        password_hash
    )
    .fetch_one(exec)
    .await?;

    Ok(credential)
}

pub async fn get_credential_by_phone(
    pool: &Pool<Postgres>,
    phone: &str,
) -> Result<Option<UserCredential>, ServerError> {
    let credential = sqlx::query_as!(
        UserCredential,
        r#"
        SELECT uc.id, uc.identity_id, uc.password_hash, uc.failed_attempts, uc.locked_until, uc.created_at, uc.updated_at
        FROM user_credential uc
        INNER JOIN user_identity ui ON ui.id = uc.identity_id
        WHERE ui.provider_id = $1
        "#,
        phone
    )
    .fetch_optional(pool)
    .await?;

    Ok(credential)
}

pub async fn find_refresh_token(
    pool: &Pool<Postgres>,
    token_hash: &str,
) -> Result<Option<RefreshToken>, ServerError> {
    let token = sqlx::query_as!(
        RefreshToken,
        r#"
        SELECT id, user_id, token_hash, user_agent, device_id, expires_at, revoked_at, created_at, last_used_at
        FROM refresh_token
        WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW()
        "#,
        token_hash
    )
    .fetch_optional(pool)
    .await?;

    Ok(token)
}

pub async fn create_refresh_token<'e, E>(
    exec: E,
    user_id: Uuid,
    token_hash: &str,
    expires_at: DateTime<Utc>,
    user_agent: Option<&str>,
    device_id: Option<&str>,
) -> Result<RefreshToken, ServerError>
where
    E: Executor<'e, Database = Postgres>,
{
    let token = sqlx::query_as!(
        RefreshToken,
        r#"
        INSERT INTO refresh_token (id, user_id, token_hash, expires_at, user_agent, device_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, token_hash, user_agent, device_id, expires_at, revoked_at, created_at, last_used_at
        "#,
        Uuid::new_v4(),
        user_id,
        token_hash,
        expires_at,
        user_agent,
        device_id
    )
    .fetch_one(exec)
    .await?;

    Ok(token)
}
