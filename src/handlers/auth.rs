use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use reqwest::StatusCode;

use crate::{
    error::ServerError,
    models::user::{PhoneLoginRequest, PhoneSignupRequest},
    state::AppState,
};

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login/phone", post(phone_login))
        .route("/signup/phone", post(phone_signup))
        .with_state(state)
}

async fn phone_login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PhoneLoginRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let token_response = state
        .auth
        .phone_login(&req.phone_number, &req.password)
        .await?;

    Ok((StatusCode::OK, Json(token_response)))
}

async fn phone_signup(
    State(_state): State<Arc<AppState>>,
    Json(_req): Json<PhoneSignupRequest>,
) -> Result<impl IntoResponse, ServerError> {
    todo!();

    Ok(())
}
