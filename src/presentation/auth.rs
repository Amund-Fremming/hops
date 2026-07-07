use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse, routing::post};

use crate::domain::{error::ServerError, state::AppState};

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/authenticate", post(authenticate))
        .route("/", post(authenticate))
        .with_state(state)
}

async fn authenticate(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServerError> {
    todo!();
    Ok(())
}
