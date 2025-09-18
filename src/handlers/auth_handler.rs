use axum::{extract::State, Json};
use crate::services::Services;
use crate::dtos::register_user_dto::RegisterUserDto;

pub async fn register_user_handler(
    State(services): State<Services>,
    Json(register_user_dto): Json<RegisterUserDto>
) {

    //TODO: Agregar control de errores.

    services.user_service.register_user(register_user_dto).await;

}
