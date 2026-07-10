use sqlx::{Executor, Pool, Postgres};

use crate::error::ServerError;
use crate::models::audit::{Action, Audit, AuditQuery, ResourceType};

pub async fn create_audit(pool: &Pool<Postgres>, audit: &Audit) -> Result<Audit, ServerError> {
    let created = sqlx::query_as!(
        Audit,
        r#"
        INSERT INTO audit_log (id, resource_id, resource_type, action, ip_address, user_agent, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, resource_id, resource_type as "resource_type: ResourceType", action as "action: Action", ip_address, user_agent, metadata, created_at
        "#,
        audit.id,
        audit.resource_id,
        &audit.resource_type as _,
        &audit.action as _,
        audit.ip_address,
        audit.user_agent,
        audit.metadata
    )
    .fetch_one(pool)
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
