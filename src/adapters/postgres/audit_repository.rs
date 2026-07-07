use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::error::ServerError;
use crate::ports::audit_repository::{Audit, AuditQuery, AuditRepository};

pub struct PostgresAuditRepository {
    pool: Pool<Postgres>,
}

impl PostgresAuditRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for PostgresAuditRepository {
    async fn create(&self, _audit: Audit) -> Result<Audit, ServerError> {
        todo!()
    }

    async fn find(&self, _query: AuditQuery) -> Result<Vec<Audit>, ServerError> {
        todo!()
    }

    async fn delete_older_than(&self, _days: i64) -> Result<u64, ServerError> {
        todo!()
    }
}
