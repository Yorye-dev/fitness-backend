use crate::auth::middleware::authorization_middleware;
use crate::presentation::handlers::user_handler::me_handler;
use crate::services::Services;
use axum::{middleware, routing::get, Router};

pub fn protected_routes(services: Services) -> Router {
    let auth_router =
        Router::new()
            .route("/me", get(me_handler))
            .layer(middleware::from_fn_with_state(
                services.clone(),
                authorization_middleware,
            ));

    Router::new().merge(auth_router).with_state(services)
}
