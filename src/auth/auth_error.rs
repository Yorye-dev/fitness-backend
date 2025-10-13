use thiserror::Error;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing authorization token")]
    MissingToken,
    #[error("Invalid authorization header")]
    InvalidHeader,
    #[error("Invalid or expired token")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidHeader => (StatusCode::BAD_REQUEST, "Invalid authorization header"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

