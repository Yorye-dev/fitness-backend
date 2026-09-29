mod auth;
mod health_routes;
mod protected;

pub use health_routes::health_routes;

use crate::{app_state::AppState, presentation::errors::api_error::ApiError};
use axum::Router;

pub fn app_routes(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/api", protected::routes(state.clone()))
        .fallback(|| async { ApiError::NotFound })
        .method_not_allowed_fallback(|| async { ApiError::MethodNotAllowed })
        .with_state(state)
}
