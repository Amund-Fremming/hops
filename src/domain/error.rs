use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum ServerError {
    #[error("Database error: {0}")]
    Database(String),
}
