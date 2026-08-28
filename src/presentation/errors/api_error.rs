use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::presentation::dto::response::api_error::ApiErrorResponse;

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
    NutritionGoalsNotFound,

    Conflict(String),

    Internal,
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,

            Self::Validation(_) => {
                StatusCode::UNPROCESSABLE_ENTITY
            }

            Self::Unauthorized
            | Self::InvalidCredentials
            | Self::InvalidToken => {
                StatusCode::UNAUTHORIZED
            }

            Self::Forbidden => {
                StatusCode::FORBIDDEN
            }

            Self::UserNotFound
            | Self::MealNotFound
            | Self::NutritionGoalsNotFound => {
                StatusCode::NOT_FOUND
            }

            Self::Conflict(_) => {
                StatusCode::CONFLICT
            }

            Self::Internal => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::Validation(_) => "VALIDATION_ERROR",

            Self::Unauthorized => "UNAUTHORIZED",
            Self::InvalidCredentials => {
                "INVALID_CREDENTIALS"
            }
            Self::Forbidden => "FORBIDDEN",
            Self::InvalidToken => "INVALID_TOKEN",

            Self::UserNotFound => "USER_NOT_FOUND",
            Self::MealNotFound => "MEAL_NOT_FOUND",
            Self::NutritionGoalsNotFound => {
                "NUTRITION_GOALS_NOT_FOUND"
            }

            Self::Conflict(_) => "CONFLICT",

            Self::Internal => {
                "INTERNAL_SERVER_ERROR"
            }
        }
    }

    fn message(&self) -> String {
        match self {
            Self::BadRequest(message)
            | Self::Validation(message)
            | Self::Conflict(message) => {
                message.clone()
            }

            Self::Unauthorized => {
                "Authentication required".to_string()
            }

            Self::InvalidCredentials => {
                "Invalid username or password".to_string()
            }

            Self::Forbidden => {
                "You do not have permission to perform this action"
                    .to_string()
            }

            Self::InvalidToken => {
                "Invalid or expired authentication token"
                    .to_string()
            }

            Self::UserNotFound => {
                "User not found".to_string()
            }

            Self::MealNotFound => {
                "Meal not found".to_string()
            }

            Self::NutritionGoalsNotFound => {
                "Nutrition goals not found".to_string()
            }

            Self::Internal => {
                "Internal server error".to_string()
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        let body = ApiErrorResponse::new(
            self.code(),
            self.message(),
        );

        (status, Json(body)).into_response()
    }
}
