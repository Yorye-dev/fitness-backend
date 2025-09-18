use axum::{Router, routing::post};
use crate::handlers::auth_handler::register_user_handler;
use crate::services::Services;

pub fn auth_routes(services: Services) -> Router {
    Router::new()
        .route("/register", post(register_user_handler))
        .with_state(services)
}

