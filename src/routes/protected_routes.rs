use axum::{
    Router,
    routing::get,
    middleware,
};
use crate::services::Services;
use crate::handlers::user_handler::me_handler;
use crate::auth::middleware::authorization_middleware;


pub fn protected_routes(services: Services) -> Router {

    // Router con todas las rutas protegidas
    let auth_router = Router::new()
        .route("/me", get(me_handler))
        //.route("/nutrition/daily", get(daily_handler))
        .layer(middleware::from_fn_with_state(
            services.clone(),
            authorization_middleware,
        ));

    // Merge en la raíz
    Router::new()
        .merge(auth_router)
        .with_state(services)
}
