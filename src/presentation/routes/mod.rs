pub mod auth_routes;
pub mod protected_routes;

use crate::presentation::routes::{auth_routes::auth_routes, protected_routes::protected_routes};
use crate::services::Services;
use axum::Router;

pub fn app_routes(services: Services) -> Router {
    Router::new()
        .nest("/auth", auth_routes(services.clone()))
        .nest("/api", protected_routes(services.clone()))
}
