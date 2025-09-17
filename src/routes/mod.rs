pub mod auth_routes;

use axum::Router;

pub fn app_routes() -> Router {
    Router::new()
        .nest("/", post() )
        .with_state() //Agregar los servicios, para que sea global
}
