use axum::{
    routing::post,
    Router,
};

use crate::app_state::AppState;
use crate::presentation::handlers::auth_handler::{
    refresh_token_handler,
    register_handler,
    sign_in_handler,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/sign_in", post(sign_in_handler))
        .route("/register", post(register_handler))
        .route("/refresh", post(refresh_token_handler))
}
