// src/routes/protected_routes.rs
use axum::{
    Router,
    routing::get,
    middleware,
};
use crate::services::Services;
use crate::auth::middleware::authorization_middleware;

pub fn protected_routes(services: Services) -> Router {
    Router::new()
        .route("/me", get(Ser))
        .layer(middleware::from_fn_with_state(
            services.clone(),
            authorization_middleware,
        ))
}
