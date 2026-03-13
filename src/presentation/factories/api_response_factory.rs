use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

pub struct ResponseFactory;

impl ResponseFactory {
    pub fn ok<T: Serialize>(data: T) -> Response {
        (StatusCode::OK, Json(data)).into_response()
    }

    pub fn created<T: Serialize>(data: T) -> Response {
        (StatusCode::CREATED, Json(data)).into_response()
    }

    pub fn no_content() -> Response {
        StatusCode::NO_CONTENT.into_response()
    }

    pub fn not_found(message: &str) -> Response {
        (StatusCode::NOT_FOUND, Json(json!({ "error": message }))).into_response()
    }

    pub fn internal_error(message: &str) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )
            .into_response()
    }
}
