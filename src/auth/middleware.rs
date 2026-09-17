use crate::app_state::AppState;
use crate::domain::errors::DomainError;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn authorization_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = match auth_header.and_then(|v| v.strip_prefix("Bearer ")) {
        Some(t) => t,
        None => {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::from("Missing or invalid Authorization header"))
                .unwrap();
        }
    };

    let claims = match state.verify_token_use_case.execute(token.to_string()).await {
        Ok(c) => c,
        Err(e) => {
            let (status, message) = match e {
                DomainError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
                DomainError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
                DomainError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            };
            return Response::builder()
                .status(status)
                .body(Body::from(message))
                .unwrap();
        }
    };

    req.extensions_mut().insert(claims);

    next.run(req).await
}
