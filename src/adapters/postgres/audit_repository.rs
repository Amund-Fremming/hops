use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::ports::audit_repository::{Audit, AuditQuery, AuditRepoError, AuditRepository};

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
    async fn create(&self, _audit: Audit) -> Result<Audit, AuditRepoError> {
        todo!()
    }

    async fn find(&self, _query: AuditQuery) -> Result<Vec<Audit>, AuditRepoError> {
        todo!()
    }

    async fn delete_older_than(&self, _days: i64) -> Result<u64, AuditRepoError> {
        todo!()
    }
}
