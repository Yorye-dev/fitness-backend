use axum::{Router, routing::post};
use crate::handlers;
use crate::handlers::auth_handler::{register_user_handler, login_user_handler};
use crate::services::Services;
use crate::auth::handlers::sign_in_handler;

pub fn auth_routes(services: Services) -> Router {
    Router::new()
        .route("/register", post(register_user_handler))
        .route("/sign_in", post(sign_in_handler))
        .with_state(services)
}
