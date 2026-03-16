use axum::{
    extract::{Extension, State},
    response::IntoResponse,
};
use uuid::Uuid;
use crate::auth::claims::Claims;
use crate::services::Services;
use crate::presentation::factories::api_response_factory::ResponseFactory;
use crate::domain::user::user::PublicUser;
use crate::domain::user::repository::UserRepository;

pub async fn me_handler(
    Extension(claims): Extension<Claims>,
    State(services): State<Services>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.subject).ok();
    
    match user_id {
        Some(id) => {
            match services.user_repository.get_user_by_id(&id).await {
                Ok(Some(user)) => {
                    let public_user: PublicUser = PublicUser::from(user);
                    ResponseFactory::ok(public_user)
                },
                Ok(None) => ResponseFactory::not_found("User not found"),
                Err(_) => ResponseFactory::internal_error("Database error"),
            }
        },
        None => ResponseFactory::bad_request("Invalid user ID in token"),
    }
}
