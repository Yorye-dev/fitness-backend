use axum::{
    extract::{State,Json},
    response::{IntoResponse, Json as JsonResponse},
    http::StatusCode,
};
use crate::errors::AuthError;
use crate::services::Services;
use crate::dtos::register_user_dto::RegisterUserDto;
use crate::dtos::login_user_dto::LoginUserDto;

// TODO: Manejo de errores, Trabajar con jwt.
//
pub async fn register_user_handler(
    State(services): State<Services>,
    Json(register_user_dto): Json<RegisterUserDto>
) {
    services.user_service.register_user(register_user_dto).await
}

pub async fn login_user_handler(
    State(services): State<Services>,
    Json(login_user_dto): Json<LoginUserDto>
) -> impl IntoResponse {

    println!("Login attempt for: {}", login_user_dto.username);
    
    let result = services.auth_service.login_user(login_user_dto).await;

    match result {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(AuthError::UserNotFound) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(AuthError::InvalidCredentials) => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    } 
}
/*
async fn sign_in_handler (
State(services): State<Services> ,
Json(login_user_dto): Json<LoginUserDto>)
-> Result <impl Into>{

}
*/
