use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;


pub async fn init_db_pg_pool(database_url: &str) ->  Result<Pool<Postgres>, sqlx::Error>{
    let pool = PgPoolOptions::new()
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Some(Duration::from_secs(600)))
        .max_lifetime(Some(Duration::from_secs(3600)))
        .connect(database_url)
        .await?;

    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?; // Select para verificar que hay conexió

    println!("Se pudo concetar con la database");
    
    Ok(pool)
} 
