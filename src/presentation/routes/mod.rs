pub mod auth_routes;
pub mod protected_routes;

use axum::Router;

use crate::app_state::AppState;
use crate::presentation::routes::{
    auth_routes::auth_routes,
    protected_routes::protected_routes,
};

pub fn app_routes(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/api", protected_routes(state.clone()))
        .with_state(state)
}
