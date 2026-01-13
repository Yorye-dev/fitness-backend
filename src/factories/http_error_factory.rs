use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use crate::services::errors::ServiceError;

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub success: bool,
    pub error: String,
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, msg: impl Into<String>) -> Self {
        Self { status, message: msg.into() }
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, msg)
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, msg)
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, msg)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ApiErrorBody {
            success: false,
            error: self.message,
        });
        (self.status, body).into_response()
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::UserNotFound => ApiError::new(StatusCode::NOT_FOUND, err.to_string()),
            ServiceError::InvalidCredentials => ApiError::unauthorized(err.to_string()),
            ServiceError::InvalidToken => ApiError::unauthorized(err.to_string()),
            ServiceError::Database(_) => ApiError::internal("Error en la base de datos"),
            ServiceError::Unexpected(e) => ApiError::internal(e),
        }
    }
}

