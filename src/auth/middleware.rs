use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    extract::{State,Json},

};
use crate::{services::Services, 
    auth::{claims::Claims, jwt::Jwt}
};


#[derive(Debug)]
pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}
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
            status_code: StatusCode::FORBIDDEN,
        })?;

    // 3️⃣ Decodificar el JWT usando auth_service
    let claims: Claims = jwt(token)
        .map_err(|_| AuthError {
            message: "Invalid or expired token".into(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    let user_id = claims.subject;

    // 4️⃣ Verificar que el usuario exista
    let exists = services.user_service.exists(&user_id)
        .await
        .map_err(|_| AuthError {
            message: "Database error".into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if !exists {
        return Err(AuthError {
            message: "User not found".into(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    // 5️⃣ Guardar el user_id en extensions para que el handler pueda accederlo
    req.extensions_mut().insert(user_id);

    // 6️⃣ Continuar con la petición
    Ok(next.run(req).await)

    // tu código aquí
    Ok(next.run(req).await)
}


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
