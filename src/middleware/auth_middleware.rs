use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use crate::{
    auth::auth_error::AuthError,
    auth::jwt::decode_jwt,
};

pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    
    //1. Leer cabecera Authorization
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AuthError::MissingToken)?;

    //2. Convertir a &str
    let auth_str = auth_header.to_str().map_err(|_| AuthError::InvalidHeader)?;

    //3. Separar "Bearer" y el token
    let mut parts = auth_str.split_whitespace();
    let scheme = parts.next().unwrap_or("");
    let token = parts.next().ok_or(AuthError::InvalidToken)?;

    if scheme != "Bearer" {
        return Err(AuthError::InvalidHeader);
    }

    //4. Decodificar JWT
    let token_data = decode_jwt(&token.to_string()).map_err(|_| AuthError::InvalidToken)?;

    //5. Guardar claims en la request
    req.extensions_mut().insert(token_data);

    // 🔹 6. Continuar al siguiente middleware o handler
    Ok(next.run(req).await)
}
