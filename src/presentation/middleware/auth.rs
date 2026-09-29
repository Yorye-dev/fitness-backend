use crate::{
    app_state::AppState,
    presentation::{authenticated_user::AuthenticatedUser, errors::api_error::ApiError},
};
use axum::{
    body::Body,
    extract::State,
    http::{Request, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn authorization_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let header = match request.headers().get(AUTHORIZATION) {
        Some(header) => header,
        None => return ApiError::Unauthorized.into_response(),
    };
    let token = match header
        .to_str()
        .ok()
        .and_then(|value| value.strip_prefix("Bearer "))
    {
        Some(token) if !token.is_empty() => token,
        _ => return ApiError::InvalidToken.into_response(),
    };
    let identity = match state.verify_token_use_case.execute(token).await {
        Ok(identity) => identity,
        Err(error) => return ApiError::authentication(error).into_response(),
    };
    request.extensions_mut().insert(AuthenticatedUser {
        user_id: identity.user_id,
    });
    next.run(request).await
}
