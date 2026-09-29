use fitness_backend::{
    app_state::AppState,
    config::{database::init_db_pg_pool, settings::Settings},
    infrastructure::health::PostgresReadinessCheck,
    presentation::{
        cors::cors_layer,
        routes::{app_routes, health_routes},
    },
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let settings = Settings::from_env()?;
    let cors = cors_layer(&settings.cors_allowed_origins)?;
    let pool = init_db_pg_pool(&settings.database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let state = AppState::new(pool.clone(), settings.secret_key);
    let router = app_routes(state)
        .merge(health_routes(Arc::new(PostgresReadinessCheck::new(
            pool.clone(),
        ))))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind(settings.bind_address).await?;
    println!(
        "{} listening on {}",
        settings.project_name, settings.bind_address
    );
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    pool.close().await;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("cannot listen for Ctrl+C");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("cannot listen for SIGTERM")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}
