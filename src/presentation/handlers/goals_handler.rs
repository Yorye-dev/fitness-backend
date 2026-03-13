use axum::{
    extract::{State, Json},
    response::IntoResponse,
};
use crate::services::Services;
use crate::dtos::sign_data_dto::SignInData;
use crate::presentation::factories::api_response_factory::ResponseFactory;

pub async fn stats_handler (
    State(services): State<Services> ,
    Json(sing_in_dto): Json<SignInData>
    ) -> impl IntoResponse 
{
    match services.auth_service.sing_in(sing_in_dto).await {
        Ok(jwt_token) => ResponseFactory::ok(jwt_token),
        Err(e) => ResponseFactory::not_found(&e.to_string()),
    }
}
