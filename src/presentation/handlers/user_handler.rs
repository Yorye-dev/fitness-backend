use axum::{
    extract::Extension,
    response::IntoResponse,
};
use serde::Serialize;
use crate::auth::claims::Claims;
use crate::presentation::factories::api_response_factory::ResponseFactory;

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

    ResponseFactory::ok(response)
}
