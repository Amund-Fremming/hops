use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::domain::{error::ServerError, state::AppState};

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/{user_id}", get(get_user))
        .with_state(*state)
}

async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ServerError> {
    match state.user_repository.get_user(user_id).await? {
        Some(user) => Ok((StatusCode::OK, Json(user))),
        None => Err(ServerError::Api(
            StatusCode::NOT_FOUND,
            "User not found".to_string(),
        )),
    }
}
