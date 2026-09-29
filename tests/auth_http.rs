mod common;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use chrono::Utc;
use common::*;
use fitness_backend::application::security::token_service::TokenService;
use fitness_backend::infrastructure::auth::jwt::JwtTokenService;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

#[sqlx::test]
async fn invalid_profile_updates_leave_user_and_goals_unchanged(pool: PgPool) {
    let app = app(pool);
    let account = register(&app, "alice").await;
    let (_, original_user) = request(&app, "GET", "/api/me", Some(&account.access), None).await;
    let (_, original_goals) = request(
        &app,
        "GET",
        "/api/nutrition/goals",
        Some(&account.access),
        None,
    )
    .await;
    let valid = json!({"weight":80,"height":180,"age":31,"activity_level":"very_active","goal":"gain_muscle"});
    for (field, value) in [
        ("weight", json!(0)),
        ("height", json!(251)),
        ("age", json!(121)),
    ] {
        let mut invalid = valid.clone();
        invalid[field] = value;
        let (status, body) =
            request(&app, "PUT", "/api/me", Some(&account.access), Some(invalid)).await;
        assert_error(
            status,
            &body,
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
        );
        let (status, user) = request(&app, "GET", "/api/me", Some(&account.access), None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(user, original_user);
        let (status, goals) = request(
            &app,
            "GET",
            "/api/nutrition/goals",
            Some(&account.access),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(goals, original_goals);
    }
}

#[sqlx::test]
async fn token_purpose_is_enforced_by_protected_routes_and_refresh(pool: PgPool) {
    let app = app(pool);
    let account = register(&app, "alice").await;
    for route in [
        "/api/me",
        "/api/nutrition/daily?date=2026-09-17",
        "/api/nutrition/goals",
        "/api/nutrition/meals",
    ] {
        let (status, body) = request(&app, "GET", route, Some(&account.refresh), None).await;
        assert_error(status, &body, StatusCode::UNAUTHORIZED, "INVALID_TOKEN");
    }
    let (status, body) = request(
        &app,
        "POST",
        "/auth/refresh",
        None,
        Some(json!({"refresh_token":account.access})),
    )
    .await;
    assert_error(status, &body, StatusCode::UNAUTHORIZED, "INVALID_TOKEN");

    let (status, body) = request(
        &app,
        "POST",
        "/auth/refresh",
        None,
        Some(json!({"refresh_token":account.refresh})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let access = body["data"]["access_token"].as_str().unwrap();
    let (status, me) = request(&app, "GET", "/api/me", Some(access), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(me["data"]["id"], account.id.to_string());
    let (status, body) = request(
        &app,
        "POST",
        "/auth/refresh",
        None,
        Some(json!({"refresh_token":access})),
    )
    .await;
    assert_error(status, &body, StatusCode::UNAUTHORIZED, "INVALID_TOKEN");
}

#[sqlx::test]
async fn legacy_tokens_without_purpose_are_rejected(pool: PgPool) {
    let app = app(pool);
    let account = register(&app, "alice").await;
    let legacy = encode(
        &Header::default(),
        &json!({
            "subject": account.id.to_string(),
            "exp": Utc::now().timestamp() + 3600,
        }),
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap();
    let (status, body) = request(&app, "GET", "/api/me", Some(&legacy), None).await;
    assert_error(status, &body, StatusCode::UNAUTHORIZED, "INVALID_TOKEN");
    let (status, body) = request(
        &app,
        "POST",
        "/auth/refresh",
        None,
        Some(json!({"refresh_token":legacy})),
    )
    .await;
    assert_error(status, &body, StatusCode::UNAUTHORIZED, "INVALID_TOKEN");
}

#[sqlx::test]
async fn login_contract_and_invalid_credentials(pool: PgPool) {
    let app = app(pool);
    register(&app, "alice").await;
    let (status, body) = request(
        &app,
        "POST",
        "/auth/sign_in",
        None,
        Some(json!({"username":"alice","password":PASSWORD})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"]["access_token"].is_string());
    assert!(body["data"]["refresh_token"].is_string());
    assert!(body.get("error").is_none());
    for username in ["alice", "unknown"] {
        let (status, body) = request(
            &app,
            "POST",
            "/auth/sign_in",
            None,
            Some(json!({"username":username,"password":"wrong-password"})),
        )
        .await;
        assert_error(
            status,
            &body,
            StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS",
        );
    }
}
#[sqlx::test]
async fn me_is_scoped_to_token_and_never_exposes_password(pool: PgPool) {
    let app = app(pool);
    let alice = register(&app, "alice").await;
    let bob = register(&app, "bob").await;
    for (account, username) in [(&alice, "alice"), (&bob, "bob")] {
        let (status, body) = request(&app, "GET", "/api/me", Some(&account.access), None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"]["username"], username);
        assert_eq!(body["data"]["id"], account.id.to_string());
        assert_eq!(body["data"]["sex"], "Male");
        assert_eq!(body["data"]["activity_level"], "ModeratelyActive");
        assert_eq!(body["data"]["goal"], "Maintain");
        assert!(!body.to_string().contains("password"));
    }
}
#[sqlx::test]
async fn protected_routes_reject_missing_invalid_and_deleted_subjects(pool: PgPool) {
    let app = app(pool);
    for route in [
        "/api/me",
        "/api/nutrition/daily?date=2026-09-17",
        "/api/nutrition/goals",
        "/api/nutrition/meals",
    ] {
        let (status, body) = request(&app, "GET", route, None, None).await;
        assert_error(status, &body, StatusCode::UNAUTHORIZED, "UNAUTHORIZED");
        let (status, body) = request(&app, "GET", route, Some("invalid.token"), None).await;
        assert_error(status, &body, StatusCode::UNAUTHORIZED, "INVALID_TOKEN");
    }
    let unknown = JwtTokenService::new(SECRET)
        .create_access_token(&Uuid::new_v4())
        .unwrap();
    let (status, body) = request(&app, "GET", "/api/me", Some(&unknown), None).await;
    assert_error(status, &body, StatusCode::UNAUTHORIZED, "INVALID_TOKEN");
}
#[sqlx::test]
async fn refresh_contract_and_registration_conflict(pool: PgPool) {
    let app = app(pool);
    let account = register(&app, "alice").await;
    let (status, body) = request(
        &app,
        "POST",
        "/auth/refresh",
        None,
        Some(json!({"refresh_token":account.refresh})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, me) = request(
        &app,
        "GET",
        "/api/me",
        body["data"]["access_token"].as_str(),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(me["data"]["id"], account.id.to_string());
    let (status, body) = request(
        &app,
        "POST",
        "/auth/refresh",
        None,
        Some(json!({"refresh_token":"invalid"})),
    )
    .await;
    assert_error(status, &body, StatusCode::UNAUTHORIZED, "INVALID_TOKEN");
    let (status, body) = request(
        &app,
        "POST",
        "/auth/register",
        None,
        Some(registration("alice")),
    )
    .await;
    assert_error(status, &body, StatusCode::CONFLICT, "CONFLICT");
    assert!(!body.to_string().contains("users_username_key"));
}
#[sqlx::test]
async fn request_errors_use_the_same_envelope(pool: PgPool) {
    let app = app(pool);
    let (status, body) = request(
        &app,
        "POST",
        "/auth/register",
        None,
        Some(json!({"username":"x"})),
    )
    .await;
    assert_error(
        status,
        &body,
        StatusCode::UNPROCESSABLE_ENTITY,
        "VALIDATION_ERROR",
    );
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/sign_in")
                .header("content-type", "application/json")
                .body(Body::from("{broken"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(body["error"]["code"], "BAD_REQUEST");
    let (status, body) = request(&app, "GET", "/auth/sign_in", None, None).await;
    assert_error(
        status,
        &body,
        StatusCode::METHOD_NOT_ALLOWED,
        "METHOD_NOT_ALLOWED",
    );
    let (status, body) = request(&app, "GET", "/unknown", None, None).await;
    assert_error(status, &body, StatusCode::NOT_FOUND, "NOT_FOUND");
}
#[sqlx::test]
async fn password_and_profile_use_cases_are_wired(pool: PgPool) {
    let app = app(pool);
    let account = register(&app, "alice").await;
    let (status, _) = request(
        &app,
        "PUT",
        "/api/me/password",
        Some(&account.access),
        Some(json!({"current_password":PASSWORD,"new_password":"another-password"})),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _) = request(
        &app,
        "POST",
        "/auth/sign_in",
        None,
        Some(json!({"username":"alice","password":"another-password"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, user) = request(&app, "PUT", "/api/me", Some(&account.access),
        Some(json!({"weight":80,"height":180,"age":31,"activity_level":"very_active","goal":"gain_muscle"}))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(user["data"]["weight"], 80.0);
    assert_eq!(user["data"]["goal"], "GainMuscle");
    assert!(!user.to_string().contains("password"));
}
