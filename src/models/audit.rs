use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "resource_type", rename_all = "snake_case")]
pub enum ResourceType {
    User,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "action", rename_all = "snake_case")]
pub enum Action {
    LoginSuccess,
    LoginFailed,
    AccountLocked,
    PasswordChange,
}

impl Action {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::AccountLocked => "account_locked",
            Self::LoginFailed => "login_failed",
            Self::LoginSuccess => "login_success",
            Self::PasswordChange => "password_change",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Audit {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub resource_type: ResourceType,
    pub action: Action,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct AuditQuery {
    pub resource_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub action: Option<Action>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Default)]
pub struct AuditBuilder {
    resource_id: Option<Uuid>,
    resource_type: Option<ResourceType>,
    action: Option<Action>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    metadata: Option<Value>,
}

impl AuditBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_id(mut self, id: Uuid) -> Self {
        self.resource_id = Some(id);
        self
    }

    pub fn resource_type(mut self, rt: ResourceType) -> Self {
        self.resource_type = Some(rt);
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    pub fn ip_address(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    pub fn metadata(mut self, meta: Value) -> Self {
        self.metadata = Some(meta);
        self
    }

    pub fn build(self) -> Audit {
        Audit {
            id: Uuid::new_v4(),
            resource_id: self.resource_id.expect("resource_id is required"),
            resource_type: self.resource_type.expect("resource_type is required"),
            action: self.action.expect("action is required"),
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            metadata: self.metadata,
            created_at: Utc::now(),
        }
    }
}
