use std::sync::Arc;

use axum::{Json, Router, middleware as axum_mw, response::IntoResponse, routing::get};
use serde_json::json;

use crate::{
    domain::state::AppState,
    presentation::{
        audit::audit_routes, auth::auth_routes, middleware::auth_mw, user::user_routes,
    },
};

pub mod audit;
pub mod auth;
pub mod middleware;
pub mod user;

async fn health() -> impl IntoResponse {
    Json(json!({ "health": "OK" }))
}

pub fn app_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/user", user_routes(state.clone()))
        .nest("/audit", audit_routes(state.clone()))
        .layer(axum_mw::from_fn_with_state(state.clone(), auth_mw))
}
