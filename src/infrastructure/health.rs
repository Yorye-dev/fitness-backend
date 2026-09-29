use crate::application::health::ReadinessCheck;
use async_trait::async_trait;
use sqlx::PgPool;
use std::time::Duration;

pub struct PostgresReadinessCheck {
    pool: PgPool,
}

impl PostgresReadinessCheck {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReadinessCheck for PostgresReadinessCheck {
    async fn is_ready(&self) -> bool {
        matches!(
            tokio::time::timeout(
                Duration::from_secs(2),
                sqlx::query("SELECT 1").execute(&self.pool),
            )
            .await,
            Ok(Ok(_))
        )
    }
}
