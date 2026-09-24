use axum::{
    extract::{Extension, State},
    response::Response,
};

use crate::app_state::AppState;
use crate::presentation::authenticated_user::AuthenticatedUser;
use crate::presentation::dto::response::user::user_response::UserResponse;
use crate::presentation::errors::api_error::ApiError;
use crate::presentation::factories::response_factory::ResponseFactory;

pub async fn me_handler(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<AuthenticatedUser>,
) -> Result<Response, ApiError> {
    let user_id = authenticated_user.user_id;

    let user = state
        .get_current_user_use_case
        .execute(user_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to retrieve user {user_id}: {error}");

            ApiError::Internal
        })?
        .ok_or(ApiError::UserNotFound)?;

    Ok(ResponseFactory::ok(UserResponse::from(user)))
}
