use crate::auth::middleware::authorization_middleware;
use crate::presentation::handlers::consumption_handler::{
    daily_progress_handler, log_consumption_handler,
};
use crate::presentation::handlers::meals_handler::{
    create_meal_handler, delete_meal_handler, get_meals_handler,
};
use crate::presentation::handlers::user_handler::me_handler;
use crate::presentation::handlers::user_routes_handler::{
    change_password_handler, get_goals_handler, update_goals_handler,
};
use crate::services::Services;
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

pub fn protected_routes(services: Services) -> Router {
    let auth_router = Router::new()
        .route("/me", get(me_handler))
        .route("/meals", post(create_meal_handler))
        .route("/meals", get(get_meals_handler))
        .route("/meals", delete(delete_meal_handler))
        .route("/goals", get(get_goals_handler))
        .route("/goals", put(update_goals_handler))
        .route("/change-password", put(change_password_handler))
        .route("/consume", post(log_consumption_handler))
        .route("/progress", get(daily_progress_handler))
        .layer(middleware::from_fn_with_state(
            services.clone(),
            authorization_middleware,
        ));

    Router::new().merge(auth_router).with_state(services)
}
