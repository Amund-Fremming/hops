use std::sync::Arc;

use axum::{Router, extract::Path, response::IntoResponse, routing::get};
use uuid::Uuid;

use crate::domain::{error::ServerError, state::AppState};

pub fn audit_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/{audit_id}", get(get_log)).with_state(state)
}

async fn get_log(Path(audit_id): Path<Uuid>) -> Result<impl IntoResponse, ServerError> {
    todo!();
    Ok(())
}
