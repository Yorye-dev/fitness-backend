use async_trait::async_trait;

/// Reports whether the dependencies required to serve requests are available.
#[async_trait]
pub trait ReadinessCheck: Send + Sync {
    async fn is_ready(&self) -> bool;
}
