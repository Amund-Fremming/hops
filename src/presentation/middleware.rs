use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use reqwest::StatusCode;

use crate::domain::state::AppState;

pub(crate) async fn auth_mw(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    Ok(next.run(req).await)
}
