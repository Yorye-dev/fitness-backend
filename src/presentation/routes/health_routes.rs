use axum::{Router, extract::State, http::StatusCode, routing::get};
use sqlx::PgPool;
use std::time::Duration;

pub fn health_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/health/ready", get(ready))
        .with_state(pool)
}

async fn ready(State(pool): State<PgPool>) -> StatusCode {
    match tokio::time::timeout(
        Duration::from_secs(2),
        sqlx::query("SELECT 1").execute(&pool),
    )
    .await
    {
        Ok(Ok(_)) => StatusCode::NO_CONTENT,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    }
}
