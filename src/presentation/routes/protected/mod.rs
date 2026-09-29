pub mod nutrition;
pub mod user;

use crate::{
    app_state::AppState,
    presentation::{errors::api_error::ApiError, middleware::auth::authorization_middleware},
};
use axum::{Router, middleware};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(user::routes())
        .nest("/nutrition", nutrition::routes())
        .fallback(|| async { ApiError::NotFound })
        .layer(middleware::from_fn_with_state(
            state,
            authorization_middleware,
        ))
}
