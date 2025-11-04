use axum::{
    Router,
    routing::get,
    middleware,
};
use crate::services::Services;
use crate::handlers::user_handler::me_handler;
use crate::auth::middleware::authorization_middleware;

pub fn protected_routes(services: Services) -> Router {
    Router::new()
        .route("/me", get(me_handler))
        .route_layer(middleware::from_fn_with_state(
            services.clone(),
            authorization_middleware,
        ))
        .with_state(services)
}

