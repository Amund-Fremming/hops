use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use uuid::Uuid;

use crate::{
    domain::{error::ServerError, state::AppState},
    ports::auth_port::Claims,
};

pub fn audit_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/{audit_id}", get(get_log))
        .with_state(state)
}

async fn get_log(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Path(audit_id): Path<Uuid>,
) -> Result<impl IntoResponse, ServerError> {
    todo!();
    Ok(())
}
