use sqlx::{Executor, Pool, Postgres};
use uuid::Uuid;

use crate::error::ServerError;
use crate::models::user::User;

pub async fn get_user(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<User>, ServerError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, phone, phone_verified, email, email_verified, given_name, family_name, avatar_url, created_at, updated_at
        FROM "user"
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn list_users(
    pool: &Pool<Postgres>,
    limit: i32,
    offset: i32,
) -> Result<Vec<User>, ServerError> {
    todo!()
}

pub async fn create_user<'e, E>(exec: E, user: &User) -> Result<User, ServerError>
where
    E: Executor<'e, Database = Postgres>,
{
    let created = sqlx::query_as!(
        User,
        r#"
        INSERT INTO "user" (id, phone, phone_verified, email, email_verified, given_name, family_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, phone, phone_verified, email, email_verified, given_name, family_name, avatar_url, created_at, updated_at
        "#,
        user.id,
        user.phone,
        user.phone_verified,
        user.email,
        user.email_verified,
        user.given_name,
        user.family_name
    )
    .fetch_one(exec)
    .await?;

    Ok(created)
}

pub async fn patch_user(pool: &Pool<Postgres>) -> Result<User, ServerError> {
    todo!()
}

pub async fn delete_user(pool: &Pool<Postgres>, id: Uuid) -> Result<bool, ServerError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM "user"
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
