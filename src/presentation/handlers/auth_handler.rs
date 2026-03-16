use axum::{
    extract::{State,Json},
    response::IntoResponse,
};
use serde::Deserialize;
use crate::services::Services;
use crate::dtos::sign_data_dto::SignInData;
use crate::presentation::factories::api_response_factory::ResponseFactory;
use crate::dtos::register_user_dto::RegisterUserDto;

#[derive(Deserialize)]
pub struct RefreshTokenDto {
    pub refresh_token: String,
}

pub async fn sign_in_handler (
    State(services): State<Services> ,
    Json(sing_in_dto): Json<SignInData>
    ) -> impl IntoResponse
{
    match services.login_use_case.execute(sing_in_dto).await {
        Ok(tokens) => ResponseFactory::ok(tokens),
        Err(e) => ResponseFactory::unauthorized(&e.to_string()),
    }
}

pub async fn register_handler(
    State(services): State<Services>,
    Json(register_user_dto): Json<RegisterUserDto>
    ) -> impl IntoResponse{
    match services.register_user_use_case.execute(register_user_dto).await{
        Ok(tokens) => ResponseFactory::ok(tokens),
        Err(e) => ResponseFactory::bad_request(&e.to_string()),
    }

}

pub async fn refresh_token_handler(
    State(services): State<Services>,
    Json(refresh_dto): Json<RefreshTokenDto>,
) -> impl IntoResponse {
    match services.refresh_token_use_case.execute(refresh_dto.refresh_token).await {
        Ok(access_token) => ResponseFactory::ok(serde_json::json!({ "access_token": access_token })),
        Err(e) => ResponseFactory::unauthorized(&e.to_string()),
    }
}
