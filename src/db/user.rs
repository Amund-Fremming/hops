use sqlx::{Executor, Pool, Postgres, query_as};
use tracing::warn;
use uuid::Uuid;

use crate::db;
use crate::error::ServerError;
use crate::models::user::{PatchUserRequest, User};

pub async fn get_user(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<User>, ServerError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, phone_number, phone_number_verified, email, email_verified, given_name, family_name, avatar_url, created_at, updated_at
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
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, phone_number, phone_number_verified, email, email_verified, given_name, family_name, avatar_url, created_at, updated_at
        FROM "user"
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit as i64,
        offset as i64
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn create_user<'e, E>(exec: E, user: &User) -> Result<User, ServerError>
where
    E: Executor<'e, Database = Postgres>,
{
    let created = sqlx::query_as!(
        User,
        r#"
        INSERT INTO "user" (id, phone_number, phone_number_verified, email, email_verified, given_name, family_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, phone_number, phone_number_verified, email, email_verified, given_name, family_name, avatar_url, created_at, updated_at
        "#,
        user.id,
        user.phone_number,
        user.phone_number_verified,
        user.email,
        user.email_verified,
        user.given_name,
        user.family_name
    )
    .fetch_one(exec)
    .await?;

    Ok(created)
}

pub async fn patch_user(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    user: &PatchUserRequest,
) -> Result<User, ServerError> {
    let mut updates = Vec::new();

    if let Some(given_name) = &user.given_name {
        updates.push(format!("given_name = '{given_name}'"));
    }

    if let Some(family_name) = &user.family_name {
        updates.push(format!("family_name = '{family_name}'"));
    }

    if let Some(avatar_url) = &user.avatar_url {
        updates.push(format!("avatar_url= '{avatar_url}'"));
    }

    if updates.is_empty() {
        warn!("User tried patching non updated fields");
        let user = db::user::get_user(pool, user_id)
            .await?
            .ok_or(ServerError::NotFound)?;

        return Ok(user);
    }

    let set_statement = updates.join(" AND ");
    let query = format!(
        r#"
        UPDATE "user"
        SET {set_statement}
        WHERE user_id = ${user_id}
        "#
    );

    let user = query_as::<_, User>(&query).fetch_one(pool).await?;

    Ok(user)
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

pub async fn is_phone_in_use(
    pool: &Pool<Postgres>,
    phone_number: &str,
) -> Result<bool, ServerError> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM "user"
            WHERE phone_number = $1 AND phone_number_verified = TRUE
        ) as "exists!"
        "#,
        phone_number
    )
    .fetch_one(pool)
    .await?;

    Ok(exists)
}
