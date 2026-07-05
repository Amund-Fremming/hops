use std::sync::Arc;

use crate::{
    domain::error::ServerError,
    ports::{auth_port::AuthPort, comms_port::CommsPort, user_repository::UserRepository},
};

#[derive(Clone)]
pub struct AppState {
    user_repo: Arc<dyn UserRepository>,
    comms: Arc<dyn CommsPort>,
    auth: Arc<dyn AuthPort>,
}

impl AppState {
    pub fn new() -> Result<Self, ServerError> {
        todo!();
    }
}
