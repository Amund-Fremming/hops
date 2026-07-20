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
    let mut conditions = Vec::new();

    if let Some(resource_id) = &query.resource_id {
        conditions.push(format!("resource_id = '{resource_id}'"));
    }

    if let Some(resource_type) = &query.resource_type {
        conditions.push(format!("resource_type = '{resource_type}'"));
    }

    if let Some(action) = &query.action {
        conditions.push(format!("action = '{}'", action.to_str()));
    }

    if let Some(from) = &query.from {
        conditions.push(format!("created_at >= '{from}'"));
    }

    if let Some(to) = &query.to {
        conditions.push(format!("created_at <= '{to}'"));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    let sql = format!(
        r#"
        SELECT id, resource_id, resource_type, action, ip_address, user_agent, metadata, created_at
        FROM audit_log
        {where_clause}
        ORDER BY created_at DESC
        LIMIT {limit} OFFSET {offset}
        "#
    );

    let logs = sqlx::query_as::<_, Audit>(&sql).fetch_all(pool).await?;

    Ok(logs)
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
