use crate::{
    application::{errors::ApplicationError, security::token_service::TokenError},
    domain::errors::{DomainError, RepositoryError},
    presentation::dto::response::api_error::ApiErrorResponse,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Validation(String),
    Unauthorized,
    InvalidCredentials,
    Forbidden,
    InvalidToken,
    UserNotFound,
    MealNotFound,
    ConsumptionNotFound,
    NutritionGoalsNotFound,
    NotFound,
    MethodNotAllowed,
    Conflict,
    UnsupportedMediaType,
    PayloadTooLarge,
    Internal,
}
impl ApiError {
    fn details(&self) -> (StatusCode, &'static str, String) {
        let (status, code, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", message.as_str()),
            Self::Validation(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "VALIDATION_ERROR",
                message.as_str(),
            ),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Authentication required",
            ),
            Self::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "Invalid username or password",
            ),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "You do not have permission to perform this action",
            ),
            Self::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "INVALID_TOKEN",
                "Invalid or expired authentication token",
            ),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "USER_NOT_FOUND", "User not found"),
            Self::MealNotFound => (StatusCode::NOT_FOUND, "MEAL_NOT_FOUND", "Meal not found"),
            Self::ConsumptionNotFound => (
                StatusCode::NOT_FOUND,
                "CONSUMPTION_NOT_FOUND",
                "Consumption not found",
            ),
            Self::NutritionGoalsNotFound => (
                StatusCode::NOT_FOUND,
                "NUTRITION_GOALS_NOT_FOUND",
                "Nutrition goals not found",
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND", "Resource not found"),
            Self::MethodNotAllowed => (
                StatusCode::METHOD_NOT_ALLOWED,
                "METHOD_NOT_ALLOWED",
                "Method not allowed",
            ),
            Self::Conflict => (
                StatusCode::CONFLICT,
                "CONFLICT",
                "Resource already exists or is still in use",
            ),
            Self::UnsupportedMediaType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "UNSUPPORTED_MEDIA_TYPE",
                "Expected application/json",
            ),
            Self::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "PAYLOAD_TOO_LARGE",
                "Request body too large",
            ),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL",
                "Internal server error",
            ),
        };
        (status, code, message.to_owned())
    }

    pub fn authentication(error: ApplicationError) -> Self {
        match error {
            ApplicationError::Domain(DomainError::UserNotFound) => Self::InvalidToken,
            error => error.into(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = self.details();
        (status, Json(ApiErrorResponse::new(code, message))).into_response()
    }
}
impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::Domain(error) => match error {
                DomainError::UserNotFound => Self::UserNotFound,
                DomainError::MealNotFound => Self::MealNotFound,
                DomainError::ConsumptionNotFound => Self::ConsumptionNotFound,
                DomainError::NutritionGoalsNotFound => Self::NutritionGoalsNotFound,
                DomainError::InvalidCredentials => Self::InvalidCredentials,
                DomainError::Validation(message) => Self::Validation(message),
            },
            ApplicationError::Repository(RepositoryError::Conflict) => Self::Conflict,
            ApplicationError::Repository(RepositoryError::NotFound) => Self::NotFound,
            ApplicationError::Token(TokenError::Invalid) => Self::InvalidToken,
            _ => Self::Internal,
        }
    }
}
impl From<axum::extract::rejection::JsonRejection> for ApiError {
    fn from(error: axum::extract::rejection::JsonRejection) -> Self {
        match error.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Self::Validation("Invalid JSON fields".into()),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Self::UnsupportedMediaType,
            StatusCode::PAYLOAD_TOO_LARGE => Self::PayloadTooLarge,
            _ => Self::BadRequest("Invalid JSON body".into()),
        }
    }
}
