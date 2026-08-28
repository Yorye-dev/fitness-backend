use axum::{
    extract::{
        Extension,
        State,
    },
    response::Response,
};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::auth::claims::Claims;
use crate::presentation::dto::response::user::user_response::UserResponse;
use crate::presentation::errors::api_error::ApiError;
use crate::presentation::factories::response_factory::ResponseFactory;

pub async fn me_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Response, ApiError> {
    let user_id = Uuid::parse_str(
        &claims.subject,
    )
    .map_err(|_| ApiError::InvalidToken)?;

    let user = state
        .get_current_user_use_case
        .execute(user_id)
        .await
        .map_err(|error| {
            eprintln!(
                "Failed to retrieve user {user_id}: {error}"
            );

            ApiError::Internal
        })?
        .ok_or(ApiError::UserNotFound)?;

    Ok(
        ResponseFactory::ok(
            UserResponse::from(user),
        ),
    )
}
