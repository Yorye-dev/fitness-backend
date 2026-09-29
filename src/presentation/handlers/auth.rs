use crate::{
    app_state::AppState,
    application::user::register_user::RegisterUserInput,
    presentation::{
        dto::{
            request::auth::{RefreshTokenRequest, RegisterUserRequest, SignInRequest},
            response::auth::{AuthTokensResponse, RefreshTokenResponse},
        },
        errors::api_error::ApiError,
        extractors::ApiJson,
        factories::response_factory::ResponseFactory,
    },
};
use axum::{extract::State, response::Response};

pub async fn sign_in(
    State(state): State<AppState>,
    ApiJson(request): ApiJson<SignInRequest>,
) -> Result<Response, ApiError> {
    request
        .validate()
        .map_err(|errors| ApiError::Validation(errors.join(", ")))?;
    let tokens = state.login_use_case.execute(request.into()).await?;
    Ok(ResponseFactory::ok(AuthTokensResponse::from(tokens)))
}
pub async fn register(
    State(state): State<AppState>,
    ApiJson(request): ApiJson<RegisterUserRequest>,
) -> Result<Response, ApiError> {
    let input = RegisterUserInput::try_from(request)
        .map_err(|errors| ApiError::Validation(errors.join(", ")))?;
    let tokens = state.register_user_use_case.execute(input).await?;
    Ok(ResponseFactory::created(AuthTokensResponse::from(tokens)))
}
pub async fn refresh(
    State(state): State<AppState>,
    ApiJson(request): ApiJson<RefreshTokenRequest>,
) -> Result<Response, ApiError> {
    let access_token = state
        .refresh_token_use_case
        .execute(&request.refresh_token)
        .await
        .map_err(ApiError::authentication)?;
    Ok(ResponseFactory::ok(RefreshTokenResponse { access_token }))
}
