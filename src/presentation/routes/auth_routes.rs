use crate::presentation::handlers::auth_handler::{register_handler, sign_in_handler};
use crate::services::Services;
use axum::{routing::post, Router};

pub fn auth_routes(services: Services) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/sign_in", post(sign_in_handler))
        .with_state(services)
}
