pub mod auth_routes;
pub mod protected_routes;

use axum::Router;

use crate::services::Services;
use crate::routes::{auth_routes::auth_routes, 
                    protected_routes::protected_routes};


pub fn app_routes(services: Services) -> Router {
    Router::new()
         .nest("/auth", auth_routes(services.clone()))
         .nest("/api", protected_routes(services.clone()))

}
