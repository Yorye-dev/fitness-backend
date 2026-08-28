use axum::{
    middleware,
    routing::get,
    Router,
};

use crate::app_state::AppState;
use crate::auth::middleware::authorization_middleware;
use crate::presentation::handlers::user_handler::me_handler;

pub fn protected_routes(
    state: AppState,
) -> Router<AppState> {
    Router::new()
        .route("/me", get(me_handler))
        .layer(
            middleware::from_fn_with_state(
                state,
                authorization_middleware,
            ),
        )
}
