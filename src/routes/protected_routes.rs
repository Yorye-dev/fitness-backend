use axum::{
    Router,
    routing::get,
    middleware,
};

use crate::services::Services;
use crate::auth::middleware::authorization_middleware;
use crate::handlers::user_handler::me_handler;

pub fn protected_routes(services: Services) -> Router {
    Router::new()
        .route("/me", get(Services::))
        .layer(middleware::from_fn_with_state(
            services.clone(),
            authorization_middleware,
        ))

    Router::new()
        .route("/me", post(me))
        .route("/sign_in", post(sign_in_handler))
        .with_state(services)
}
