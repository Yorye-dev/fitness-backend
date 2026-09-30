use crate::presentation::handlers::daily_tracking;
use crate::{app_state::AppState, presentation::handlers::training};
use axum::{
    Router,
    routing::{get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/progress", get(training::progress))
        .route("/sessions", post(daily_tracking::start_session))
        .route("/sessions/{id}", put(daily_tracking::save_session))
        .route("/routines", get(training::list))
        .route(
            "/routines/{id}",
            put(training::save).delete(training::archive),
        )
        .route("/week", get(training::week).put(training::save_week))
        .route("/daily", get(training::daily))
}
