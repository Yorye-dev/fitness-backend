use crate::{app_state::AppState, presentation::handlers::auth};
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sign_in", post(auth::sign_in))
        .route("/register", post(auth::register))
        .route("/refresh", post(auth::refresh))
}
