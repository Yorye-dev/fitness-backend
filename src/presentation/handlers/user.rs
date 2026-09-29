use crate::{
    app_state::AppState,
    presentation::{
        authenticated_user::AuthenticatedUser,
        dto::{
            request::user::{
                change_password_request::ChangePasswordRequest,
                update_user_request::UpdateUserRequest,
            },
            response::user::user_response::UserResponse,
        },
        errors::api_error::ApiError,
        extractors::ApiJson,
        factories::response_factory::ResponseFactory,
    },
};
use axum::{
    extract::{Extension, State},
    response::Response,
};

pub async fn me(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Response, ApiError> {
    Ok(ResponseFactory::ok(UserResponse::from(
        state
            .get_current_user_use_case
            .execute(user.user_id)
            .await?,
    )))
}
pub async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(request): ApiJson<UpdateUserRequest>,
) -> Result<Response, ApiError> {
    let updated = state
        .update_user_use_case
        .execute(user.user_id, request.try_into()?)
        .await?;
    Ok(ResponseFactory::ok(UserResponse::from(updated)))
}
pub async fn change_password(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(request): ApiJson<ChangePasswordRequest>,
) -> Result<Response, ApiError> {
    state
        .change_password_use_case
        .execute(
            user.user_id,
            &request.current_password,
            &request.new_password,
        )
        .await?;
    Ok(ResponseFactory::no_content())
}
