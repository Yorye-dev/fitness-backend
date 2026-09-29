use crate::{
    app_state::AppState,
    presentation::handlers::nutrition::{daily, goals, meals},
};
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/daily", get(daily::get_daily))
        .route("/goals", get(goals::get).put(goals::update))
        .route("/meals", get(meals::list).post(meals::create))
        .route(
            "/meals/{id}",
            get(meals::get).put(meals::update).delete(meals::delete),
        )
}
