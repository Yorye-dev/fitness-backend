use std::env;

use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

use dotenv::dotenv;

pub mod application;
pub mod app_state;
pub mod auth;
pub mod config;
pub mod domain;
pub mod dtos;
pub mod errors;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use app_state::AppState;
use config::database::init_db_pg_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL")
            .expect("Falta DATABASE_URL en el entorno");

    let app_url =
        env::var("APP_URL")
            .expect("Falta APP_URL en el entorno");

    let project_name =
        env::var("PROJECT_NAME")
            .expect("Falta PROJECT_NAME en el entorno");

    let secret_key =
        env::var("SECRET_KEY")
            .expect("Falta SECRET_KEY en el entorno");

    let pool =
        init_db_pg_pool(&database_url)
            .await
            .expect("No se pudo conectar a PostgreSQL");

    let app_state =
        AppState::new(
            pool,
            secret_key,
        );

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<HeaderValue>()
                .expect("Origen CORS inválido"),
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

    let app =
        presentation::routes::app_routes(
            app_state,
        )
        .layer(cors);

    let listener =
        tokio::net::TcpListener::bind(
            &app_url,
        )
        .await
        .expect("No se pudo abrir el puerto HTTP");

    println!(
        "{} corriendo en: {}",
        project_name,
        app_url,
    );

    axum::serve(listener, app)
        .await
        .expect("Error ejecutando el servidor");
}
