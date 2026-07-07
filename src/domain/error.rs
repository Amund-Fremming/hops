use axum::response::IntoResponse;
use reqwest::StatusCode;
use tracing::{error, warn};

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("API error: {0} - {1}")]
    Api(StatusCode, String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(msg) => {
                error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            Self::Api(sc, msg) => {
                warn!("API error: {} - {}", sc, msg);
                (sc, msg)
            }
        }
        .into_response()
    }
}
