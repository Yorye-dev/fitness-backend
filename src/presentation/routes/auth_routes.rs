use axum::{Router, routing::post};
use crate::handlers::auth_handler::{sign_in_handler, register_handler};
use crate::services::Services;

pub fn auth_routes(services: Services) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/sign_in", post(sign_in_handler))
        .with_state(services)
}
