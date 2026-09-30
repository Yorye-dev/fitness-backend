use crate::{
    app_state::AppState,
    presentation::handlers::nutrition::{consumptions, daily, goals, meals},
};
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/daily", get(daily::get_daily))
        .route("/consumptions", post(consumptions::create))
        .route(
            "/consumptions/{id}",
            delete(consumptions::delete).put(consumptions::update),
        )
        .route("/goals", get(goals::get).put(goals::update))
        .route("/meals", get(meals::list).post(meals::create))
        .route(
            "/meals/{id}",
            get(meals::get).put(meals::update).delete(meals::delete),
        )
}
