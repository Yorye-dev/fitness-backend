use axum::{
    extract::{State,Json},
    response::{IntoResponse, Json as JsonResponse},
    http::StatusCode,
};
use crate::services::Services;
use crate::dtos::login_user_dto::LoginUserDto;
use crate::factories::api_response_factory::ResponseFactory;

pub async fn sign_in_handler (
    State(services): State<Services> ,
    Json(sing_in_dto): Json<LoginUserDto>
    ) -> impl IntoResponse 
{
    //let tokenData = services.auth_service.login_user(login_user_dto);
    match services.auth_service.sing_in(sing_in_dto).await {
        Ok(jwt_token) => ResponseFactory::ok(jwt_token),
        Err(e) => ResponseFactory::not_found(&e.to_string()),
    }
}
