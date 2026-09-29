use std::{env, fs};

use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

use dotenv::dotenv;

pub mod app_state;
pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use app_state::AppState;
use config::database::init_db_pg_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env_or_file("DATABASE_URL", "DATABASE_URL_FILE");

    // BIND_ADDRESS describes the socket this process listens on. APP_URL is
    // accepted as a temporary fallback for existing local configurations.
    let bind_address =
        env::var("BIND_ADDRESS")
            .or_else(|_| env::var("APP_URL"))
            .expect("Falta BIND_ADDRESS en el entorno");

    let project_name =
        env::var("PROJECT_NAME")
            .expect("Falta PROJECT_NAME en el entorno");

    let secret_key = env_or_file("SECRET_KEY", "SECRET_KEY_FILE");

    let pool = init_db_pg_pool(&database_url)
        .await
        .expect("No se pudo conectar a PostgreSQL");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("No se pudieron aplicar las migraciones de PostgreSQL");

    println!("Migraciones de PostgreSQL aplicadas");

    let health_routes = presentation::routes::health_routes::health_routes(pool.clone());

    let app_state =
        AppState::new(pool, secret_key);

    let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .expect("CORS_ALLOWED_ORIGINS contiene un origen inválido")
        })
        .collect::<Vec<_>>();

    let cors = CorsLayer::new()
        .allow_origin(cors_allowed_origins)
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
        presentation::routes::app_routes(app_state)
            .merge(health_routes)
            .layer(cors);

    let listener =
        tokio::net::TcpListener::bind(&bind_address)
            .await
            .expect("No se pudo abrir el puerto HTTP");

    println!(
        "{} corriendo en: {}",
        project_name,
        bind_address,
    );

    axum::serve(listener, app)
        .await
        .expect("Error ejecutando el servidor");
}

/// Reads a configuration value directly from the environment or from a mounted
/// secret file. The `_FILE` form takes precedence when both are present.
fn env_or_file(variable: &str, file_variable: &str) -> String {
    let value = match env::var(file_variable) {
        Ok(path) => fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("No se pudo leer {file_variable} ({path}): {error}")),
        Err(_) => env::var(variable).unwrap_or_else(|_| {
            panic!("Falta {variable} o {file_variable} en el entorno")
        }),
    };

    let value = value.trim().to_owned();
    if value.is_empty() {
        panic!("{variable} no puede estar vacío");
    }

    value
}
