use axum::{Json, Router, routing::get};
use hops::config::{AppConfig, CONFIG};
use serde::Serialize;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Serialize)]
struct HealthResponse {
    health: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { health: "OK" })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("hops=info".parse().unwrap()))
        .init();

    let app = Router::new().route("/health", get(health));

    let server_cfg = CONFIG.server.clone();
    let addr = format!("{}:{}", server_cfg.address, server_cfg.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
