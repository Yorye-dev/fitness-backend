use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    extract::State,
};
use crate::app_state::AppState;
use crate::domain::errors::DomainError;

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

    let claims = match state
        .verify_token_use_case
        .execute(token.to_string())
        .await
    {
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

/*
pub async fn authorization_middleware(
    State(services): State<Services>,
    mut req: Request<Body>,            // body concreto Body
    next: Next,
) -> Result<Response, AuthError> {

    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(AuthError {
            message: "Missing Authorization header".into(),
            status_code: StatusCode::FORBIDDEN,
        })?;

     // 2️⃣ Verificar formato Bearer
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError {
            message: "Invalid token format".into(),
            status_code: StatusCode::FORBIDDEN
        })?;

    let claims: Claims = services.auth_service.get_claims_if_valid(token.to_string()).await.map_err(|_| AuthError{
            message: "Missing Authorization header".into(),
            status_code: StatusCode::FORBIDDEN,
    })?;
    // 4️⃣ Insertar el usuario autenticado en las extensiones
    req.extensions_mut().insert(claims);

    // 5️⃣ Continuar la petición
    Ok(next.run(req).await)
}
*/

/*
pub async fn authorization_middleware<B>(
    State(services): State<Services>,
    mut req: Request<B>, 
    next: Next) -> Result<Response<Body>, AuthError>{

    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN
        })?,
        None => return Err(AuthError {
            message: "Please add the JWT token to the header".to_string(),
            status_code: StatusCode::FORBIDDEN
        }),
    };
    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(), header.next());
    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => return Err(AuthError {
            message: "Unable to decode token".to_string(),
            status_code: StatusCode::UNAUTHORIZED
        }),
    };
    // Fetch the user details from the database
    let current_user = match retrieve_user_by_email(&token_data.claims.email) {
        Some(user) => user,
        None => return Err(AuthError {
            message: "You are not an authorized user".to_string(),
            status_code: StatusCode::UNAUTHORIZED
        }),
    };
    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}


*/
