use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::domain::{error::ServerError, state::AppState};

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/authenticate", post(authenticate)).with_state(state)
}

async fn authenticate() -> Result<impl IntoResponse, ServerError> {
    todo!();
    Ok(())
}
