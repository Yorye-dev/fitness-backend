pub mod nutrition;
pub mod training;
pub mod user;

use crate::presentation::handlers::daily_tracking;
use crate::{
    app_state::AppState,
    presentation::{errors::api_error::ApiError, middleware::auth::authorization_middleware},
};
use axum::routing::{delete, get, post, put};
use axum::{Router, middleware};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/water/daily", get(daily_tracking::water))
        .route("/water/intakes", post(daily_tracking::add_water))
        .route("/water/intakes/{id}", delete(daily_tracking::remove_water))
        .route("/water/goal", put(daily_tracking::water_goal))
        .merge(user::routes())
        .nest("/nutrition", nutrition::routes())
        .nest("/training", training::routes())
        .fallback(|| async { ApiError::NotFound })
        .layer(middleware::from_fn_with_state(
            state,
            authorization_middleware,
        ))
}
