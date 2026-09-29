use axum::{
    Router,
    body::Body,
    http::{
        Request,
        header::{ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN},
    },
    routing::get,
};
use fitness_backend::presentation::cors::cors_layer;
use tower::ServiceExt;

#[tokio::test]
async fn cors_only_allows_configured_development_origins() {
    let app = Router::new()
        .route("/probe", get(|| async { "ok" }))
        .layer(cors_layer(&["http://localhost:5173".into()]).unwrap());
    for (origin, allowed) in [
        ("http://localhost:5173", true),
        ("https://other.example", false),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("OPTIONS")
                    .uri("/probe")
                    .header("origin", origin)
                    .header("access-control-request-method", "GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            response.headers().contains_key(ACCESS_CONTROL_ALLOW_ORIGIN),
            allowed
        );
        assert!(
            response
                .headers()
                .contains_key(ACCESS_CONTROL_ALLOW_METHODS)
        );
    }
}
#[tokio::test]
async fn empty_production_cors_still_serves_same_origin_requests() {
    let app = Router::new()
        .route("/probe", get(|| async { "ok" }))
        .layer(cors_layer(&[]).unwrap());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/probe")
                .header("origin", "https://other.example")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    assert!(!response.headers().contains_key(ACCESS_CONTROL_ALLOW_ORIGIN));
}
