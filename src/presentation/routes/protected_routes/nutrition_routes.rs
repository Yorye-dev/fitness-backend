use axum::{
    routing::get,
    Router,
};

use crate::app_state::AppState;
use crate::presentation::handlers::nutrition_handler::get_daily_nutrition_handler;

pub fn nutrition_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/daily",
            get(get_daily_nutrition_handler),
        )
}
