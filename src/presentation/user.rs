use std::sync::Arc;

use axum::{Extension, Json, Router, extract::State, response::IntoResponse, routing::get};
use reqwest::StatusCode;

use crate::{
    domain::{error::ServerError, state::AppState},
    ports::auth_port::Claims,
};

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/me", get(me)).with_state(state)
}

async fn me(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, ServerError> {
    let user_id = claims.user_id()?;

    match state.user_repository.get_user(user_id).await? {
        Some(user) => Ok((StatusCode::OK, Json(user))),
        None => Err(ServerError::NotFound),
    }
}
