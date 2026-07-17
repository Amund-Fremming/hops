use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::ports::comms::CommsPort;
use crate::ports::crypto::CryptoPort;
use crate::services::auth::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub auth: Arc<AuthService>,
    pub comms: Arc<dyn CommsPort>,
    pub crypto: Arc<dyn CryptoPort>,
}

impl AppState {
    pub fn new(
        pool: Pool<Postgres>,
        auth: Arc<AuthService>,
        comms: Arc<dyn CommsPort>,
        crypto: Arc<dyn CryptoPort>,
    ) -> Self {
        Self {
            pool,
            auth,
            comms,
            crypto,
        }
    }
}
