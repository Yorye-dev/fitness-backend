use crate::application::health::ReadinessCheck;
use axum::{Router, extract::State, http::StatusCode, routing::get};
use std::sync::Arc;

pub fn health_routes(check: Arc<dyn ReadinessCheck>) -> Router {
    Router::new()
        .route("/health/ready", get(ready))
        .with_state(check)
}

async fn ready(State(check): State<Arc<dyn ReadinessCheck>>) -> StatusCode {
    if check.is_ready().await {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}
