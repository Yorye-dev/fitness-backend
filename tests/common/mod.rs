#![allow(dead_code)]
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use fitness_backend::{app_state::AppState, presentation::routes::app_routes};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

pub const SECRET: &str = "integration-only-key-not-used-by-any-deployment";
pub const PASSWORD: &str = "integration-password";

pub fn app(pool: PgPool) -> Router {
    app_routes(AppState::new(pool, SECRET.into()))
}
pub async fn request(
    app: &Router,
    method: &str,
    uri: &str,
    token: Option<&str>,
    data: Option<Value>,
) -> (StatusCode, Value) {
    let mut request = Request::builder().method(method).uri(uri);
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let body = match data {
        Some(data) => {
            request = request.header("content-type", "application/json");
            Body::from(data.to_string())
        }
        None => Body::empty(),
    };
    let response = app
        .clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), 1_048_576).await.unwrap();
    let value = if body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body).unwrap()
    };
    (status, value)
}
pub fn registration(username: &str) -> Value {
    json!({"username":username,"plain_password":PASSWORD,"sex":"male","weight":75.0,
        "height":175,"age":30,"activity_level":"moderately_active","goal":"maintain"})
}
pub struct Account {
    pub id: Uuid,
    pub access: String,
    pub refresh: String,
}
pub async fn register(app: &Router, username: &str) -> Account {
    let (status, body) = request(
        app,
        "POST",
        "/auth/register",
        None,
        Some(registration(username)),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    let access = body["data"]["access_token"].as_str().unwrap().to_owned();
    let refresh = body["data"]["refresh_token"].as_str().unwrap().to_owned();
    let (status, me) = request(app, "GET", "/api/me", Some(&access), None).await;
    assert_eq!(status, StatusCode::OK);
    Account {
        id: me["data"]["id"].as_str().unwrap().parse().unwrap(),
        access,
        refresh,
    }
}
pub fn assert_error(status: StatusCode, body: &Value, expected: StatusCode, code: &str) {
    assert_eq!(status, expected, "{body}");
    assert_eq!(body["error"]["code"], code);
    assert!(body["error"]["message"].is_string());
    assert!(body.get("data").is_none());
}
