use std::sync::Arc;

use axum::{Extension, Json, Router, extract::State, response::IntoResponse, routing::get};
use reqwest::StatusCode;

use crate::{
    error::ServerError,
    state::AppState,
    models::auth::Claims,
};

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/me", get(me)).with_state(state)
}

async fn me(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, ServerError> {
    let user_id = claims.user_id()?;

    match crate::db::users::get_user(&state.pool, user_id).await? {
        Some(user) => Ok((StatusCode::OK, Json(user))),
        None => Err(ServerError::NotFound),
    }
}
