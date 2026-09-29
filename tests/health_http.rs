mod common;

use axum::http::StatusCode;
use common::request;
use fitness_backend::{
    infrastructure::health::PostgresReadinessCheck, presentation::routes::health_routes,
};
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test]
async fn readiness_reports_available_and_closed_database_without_authentication(pool: PgPool) {
    let app = health_routes(Arc::new(PostgresReadinessCheck::new(pool.clone())));
    let (status, body) = request(&app, "GET", "/health/ready", None, None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert!(body.is_null());

    pool.close().await;
    let (status, body) = request(&app, "GET", "/health/ready", None, None).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(body.is_null());
}
