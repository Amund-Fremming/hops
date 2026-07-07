use std::sync::Arc;

use crate::ports::{
    audit_repository::AuditRepository, auth_port::AuthPort, comms_port::CommsPort,
    user_repository::UserRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub audit_repository: Arc<dyn AuditRepository>,
    pub comms: Arc<dyn CommsPort>,
    pub auth: Arc<dyn AuthPort>,
}

impl AppState {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        audit_repository: Arc<dyn AuditRepository>,
        auth: Arc<dyn AuthPort>,
        comms: Arc<dyn CommsPort>,
    ) -> Self {
        Self {
            user_repository,
            audit_repository,
            auth,
            comms,
        }
    }
}
