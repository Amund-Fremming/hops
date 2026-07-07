use std::sync::Arc;

use axum::{
    Json, Router, extract::State, middleware as axum_mw, response::IntoResponse, routing::get,
};
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

async fn test() -> impl IntoResponse {
    Json(json!({ "health": "OK" }))
}

async fn well_known_jwks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.auth.get_jwks().clone())
}

pub fn app_routes(state: Arc<AppState>) -> Router {
    let public_routes: Router = Router::new()
        .route("/health", get(health))
        .route("/.well-known/jwks.json", get(well_known_jwks))
        .with_state(state.clone());

    let protected_routes: Router = Router::new()
        .route("/test", get(test))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/user", user_routes(state.clone()))
        .nest("/audit", audit_routes(state.clone()))
        .layer(axum_mw::from_fn_with_state(state, auth_mw));

    Router::new().merge(public_routes).merge(protected_routes)
}
