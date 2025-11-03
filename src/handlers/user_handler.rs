use axum::{
    extract::Extension,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use crate::auth::claims::Claims;

#[derive(Serialize)]
struct MeResponse {
    user_id: String,
    exp: usize,
}

pub async fn me_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    let response = MeResponse {
        user_id: claims.subject,
        exp: claims.exp,
    };

    Json(response)
}

