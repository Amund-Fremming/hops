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
