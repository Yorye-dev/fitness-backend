use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("password service failure")]
pub struct PasswordError;

#[async_trait]
pub trait PasswordService: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, PasswordError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, PasswordError>;
}
