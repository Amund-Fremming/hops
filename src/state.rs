use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::ports::comms::CommsPort;
use crate::services::auth::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub auth: Arc<AuthService>,
    pub comms: Arc<dyn CommsPort>,
}

impl AppState {
    pub fn new(
        pool: Pool<Postgres>,
        auth: Arc<AuthService>,
        comms: Arc<dyn CommsPort>,
    ) -> Self {
        Self {
            pool,
            auth,
            comms,
        }
    }
}
