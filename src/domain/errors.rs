use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("user not found")]
    UserNotFound,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("password hashing error")]
    HashingError,

    #[error("generate token error")]
    GenerateTokenError,

    #[error("unauthorized token")]
    Unauthorized,

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("missing authorization token")]
    MissingToken,

    #[error("invalid authorization header")]
    InvalidHeader,

    #[error("invalid or expired token")]
    InvalidToken,
}

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, String) = match self {
            DomainError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                "Missing authorization token".to_string(),
            ),
            DomainError::InvalidHeader => (
                StatusCode::BAD_REQUEST,
                "Invalid authorization header".to_string(),
            ),
            DomainError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            ),
            DomainError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            DomainError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            DomainError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            DomainError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            DomainError::HashingError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Password hashing error".to_string(),
            ),
            DomainError::GenerateTokenError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Generate token error".to_string(),
            ),
            DomainError::ValidationError(e) => (StatusCode::BAD_REQUEST, e),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
