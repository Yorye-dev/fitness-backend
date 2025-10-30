use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
};
use crate::{Services, auth::claims::Claims};

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl FromRequestParts<Services> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        services: &Services, // Axum ya te pasa el state directamente
    ) -> Result<Self, Self::Rejection> {

        // 1️⃣ Leer la cabecera Authorization
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".into()))?;

        // 2️⃣ Verificar formato Bearer
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid token format".into()))?;

        // 3️⃣ Decodificar el token usando auth_service
        let claims = services.auth_service.verify_token(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token".into()))?;

        let user_id = claims.subject;

        // 4️⃣ Verificar que el usuario exista en la base de datos
        let exists = services.user_service.exists(&user_id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

        if !exists {
            return Err((StatusCode::UNAUTHORIZED, "User not found".into()));
        }

        Ok(AuthUser { user_id })
    }
}

