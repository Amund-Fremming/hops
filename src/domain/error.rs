use axum::response::IntoResponse;
use reqwest::StatusCode;
use tracing::{error, warn};

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,

    #[error("Conflict")]
    Conflict,

    #[error("API error: {0} - {1}")]
    Api(StatusCode, String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(err) => {
                error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            Self::NotFound => {
                warn!("Resource not found");
                (StatusCode::NOT_FOUND, "Not found".to_string())
            }
            Self::Conflict => (StatusCode::CONFLICT, "Conflict".to_string()),
            Self::Api(sc, msg) => {
                warn!("API error: {} - {}", sc, msg);
                (sc, msg)
            }
        }
        .into_response()
    }
}
