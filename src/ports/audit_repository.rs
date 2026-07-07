use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audit {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub resource_type: String,
    pub action: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone)]
pub struct AuditQuery {
    pub id: Option<Uuid>,
    pub resource_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub action: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug)]
pub enum AuditRepoError {
    NotFound,
    DatabaseError(String),
}

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn create(&self, audit: Audit) -> Result<Audit, AuditRepoError>;

    async fn find(&self, query: AuditQuery) -> Result<Vec<Audit>, AuditRepoError>;

    async fn delete_older_than(&self, days: i64) -> Result<u64, AuditRepoError>;
}
