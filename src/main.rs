pub mod config;
pub mod dtos;
pub mod services;
pub mod errors;
pub mod auth;
pub mod domain;
pub mod application;
pub mod presentation;
pub mod shared;
pub mod infrastructure;
use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

use dotenv::dotenv;
use services::Services;
use config::database::init_db_pg_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Error en .env falta DATABASE_URL");
    let cors = CorsLayer::new()
    .allow_origin(
        "http://localhost:5173"
            .parse::<HeaderValue>()
            .unwrap(),
    )
    .allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
    ])
    .allow_headers(Any);
    let app_url = env::var("APP_URL").expect("Error en .env falta APP_URL");
    let project_name = env::var("PROJECT_NAME").expect("Error en .env falta PROJECT_NAME");
    let secret_key = env::var("SECRET_KEY").expect("Error en .env falta SECRET_KEY");

    let _pool = init_db_pg_pool(&database_url).await.unwrap();
    let services = Services::new(_pool, secret_key);
    
    println!("{} corriendo en: {}", project_name, app_url);

    let app = presentation::routes::app_routes(services.clone()).layer(cors);
    let listener = tokio::net::TcpListener::bind(app_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
