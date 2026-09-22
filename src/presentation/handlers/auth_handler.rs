use axum::{
    extract::{Json, State},
    response::Response,
};

use crate::app_state::AppState;
use crate::application::auth::login::LoginInput;
use crate::application::user::register_user::RegisterUserInput;
use crate::domain::errors::DomainError;
use crate::presentation::dto::request::auth::{
    RefreshTokenRequest, RegisterUserRequest, SignInRequest,
};
use crate::presentation::errors::api_error::ApiError;
use crate::presentation::factories::response_factory::ResponseFactory;

pub async fn sign_in_handler(
    State(state): State<AppState>,
    Json(sign_in_request): Json<SignInRequest>,
) -> Result<Response, ApiError> {
    if let Err(errors) = sign_in_request.validate() {
        return Err(ApiError::Validation(errors.join(", ")));
    }

    let input = LoginInput::from(sign_in_request);

    let tokens = state
        .login_use_case
        .execute(input)
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
    Json(register_request): Json<RegisterUserRequest>,
) -> Result<Response, ApiError> {
    let input = RegisterUserInput::try_from(register_request)
        .map_err(|errors| ApiError::Validation(errors.join(", ")))?;

    let tokens = state
        .register_user_use_case
        .execute(input)
        .await
        .map_err(ApiError::from)?;

    Ok(ResponseFactory::created(tokens))
}

pub async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(refresh_request): Json<RefreshTokenRequest>,
) -> Result<Response, ApiError> {
    let access_token = state
        .refresh_token_use_case
        .execute(refresh_request.refresh_token)
        .await
        .map_err(|error| match error {
            DomainError::InvalidToken | DomainError::UserNotFound => ApiError::InvalidToken,

            error => ApiError::from(error),
        })?;

    Ok(ResponseFactory::ok(serde_json::json!({
        "access_token": access_token
    })))
}
