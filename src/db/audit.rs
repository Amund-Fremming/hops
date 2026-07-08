use sqlx::{Executor, Pool, Postgres};

use crate::error::ServerError;
use crate::models::audit::{Audit, AuditQuery};

pub async fn create<'e, E>(exec: E, audit: &Audit) -> Result<Audit, ServerError>
where
    E: Executor<'e, Database = Postgres>,
{
    let created = sqlx::query_as!(
        Audit,
        r#"
        INSERT INTO audit_log (id, user_id, resource_id, resource_type, action, ip_address, user_agent, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, resource_id, resource_type, action, ip_address, user_agent, metadata, created_at
        "#,
        audit.id,
        audit.user_id,
        audit.resource_id,
        audit.resource_type,
        audit.action,
        audit.ip_address,
        audit.user_agent,
        audit.metadata
    )
    .fetch_one(exec)
    .await?;

    Ok(created)
}

pub async fn list(pool: &Pool<Postgres>, query: AuditQuery) -> Result<Vec<Audit>, ServerError> {
    todo!()
}

pub async fn delete_older_than<'e, E>(exec: E, days: i64) -> Result<u64, ServerError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query!(
        r#"
        DELETE FROM audit_log
        WHERE created_at < NOW() - make_interval(days => $1::int)
        "#,
        days as i32
    )
    .execute(exec)
    .await?;

    Ok(result.rows_affected())
}
