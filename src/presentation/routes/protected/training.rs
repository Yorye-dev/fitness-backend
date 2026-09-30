use crate::{app_state::AppState, presentation::handlers::training};
use axum::{
    Router,
    routing::{get, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/routines", get(training::list))
        .route(
            "/routines/{id}",
            put(training::save).delete(training::archive),
        )
        .route("/week", get(training::week).put(training::save_week))
        .route("/daily", get(training::daily))
}
