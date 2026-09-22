use axum::{
    extract::{Json, State},
    response::Response,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::domain::errors::DomainError;
use crate::dtos::register_user_dto::RegisterUserDto;
use crate::dtos::sign_data_dto::SignInData;
use crate::presentation::errors::api_error::ApiError;
use crate::presentation::factories::response_factory::ResponseFactory;

#[derive(Debug, Deserialize)]
pub struct RefreshTokenDto {
    pub refresh_token: String,
}

pub async fn sign_in_handler(
    State(state): State<AppState>,
    Json(sign_in_dto): Json<SignInData>,
) -> Result<Response, ApiError> {
    if let Err(errors) = sign_in_dto.validate() {
        return Err(ApiError::Validation(errors.join(", ")));
    }

    let tokens = state
        .login_use_case
        .execute(sign_in_dto)
        .await
        .map_err(|error| match error {
            DomainError::UserNotFound | DomainError::InvalidCredentials => {
                ApiError::InvalidCredentials
            }
            error => ApiError::from(error),
        })?;

    Ok(ResponseFactory::ok(tokens))
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(register_user_dto): Json<RegisterUserDto>,
) -> Result<Response, ApiError> {
    if let Err(errors) = register_user_dto.validate() {
        return Err(ApiError::Validation(errors.join(", ")));
    }

    let tokens = state
        .register_user_use_case
        .execute(register_user_dto)
        .await
        .map_err(ApiError::from)?;

    Ok(ResponseFactory::created(tokens))
}

pub async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(refresh_dto): Json<RefreshTokenDto>,
) -> Result<Response, ApiError> {
    let access_token = state
        .refresh_token_use_case
        .execute(refresh_dto.refresh_token)
        .await
        .map_err(|error| match error {
            DomainError::InvalidToken | DomainError::UserNotFound => ApiError::InvalidToken,
            error => ApiError::from(error),
        })?;

    Ok(ResponseFactory::ok(serde_json::json!({
        "access_token": access_token
    })))
}
