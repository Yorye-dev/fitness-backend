use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::auth::jwt;
/*
pub async fn authorize<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    if auth_header.is_none() || !auth_header.unwrap().starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth_header.unwrap().trim_start_matches("Bearer ").to_string();

    let token_data = jwt::decode_jwt(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Guardamos los claims en la request
    req.extensions_mut().insert(&token_data);

    Ok(next.run(req).await)
}*/

