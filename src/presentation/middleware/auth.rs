use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::domain::errors::DomainError;
use crate::presentation::authenticated_user::AuthenticatedUser;
use crate::presentation::errors::api_error::ApiError;

pub async fn authorization_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = match req.headers().get(AUTHORIZATION) {
        Some(header) => header,
        None => {
            return ApiError::Unauthorized.into_response();
        }
    };

    let auth_header = match auth_header.to_str() {
        Ok(header) => header,
        Err(_) => {
            return ApiError::BadRequest(
                "Invalid authorization header".to_string(),
            )
            .into_response();
        }
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(token) if !token.is_empty() => token,
        _ => {
            return ApiError::BadRequest(
                "Invalid authorization header".to_string(),
            )
            .into_response();
        }
    };

    let claims = match state
        .verify_token_use_case
        .execute(token.to_string())
        .await
    {
        Ok(claims) => claims,

        Err(error) => {
            let api_error = match error {
                DomainError::InvalidToken | DomainError::UserNotFound => {
                    ApiError::InvalidToken
                }

                error => ApiError::from(error),
            };

            return api_error.into_response();
        }
    };

    let user_id = match Uuid::parse_str(&claims.subject) {
        Ok(user_id) => user_id,
        Err(_) => {
            return ApiError::InvalidToken.into_response();
        }
    };

    req.extensions_mut()
        .insert(AuthenticatedUser { user_id });

    next.run(req).await
}
