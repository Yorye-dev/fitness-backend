use crate::{app_state::AppState, presentation::handlers::user};
use axum::{
    Router,
    routing::{get, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(user::me).put(user::update))
        .route("/me/password", put(user::change_password))
}
