use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use reqwest::StatusCode;

use crate::domain::{error::ServerError, state::AppState, user::LoginRequest};

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/authenticate", post(authenticate))
        .route("/", post(authenticate))
        .with_state(state)
}

async fn authenticate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let token_response = state
        .auth
        .authenticate(&req.username, &req.password)
        .await?;

    Ok((StatusCode::OK, Json(token_response)))
}
