use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenIdentity {
    pub user_id: Uuid,
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("invalid or expired token")]
    Invalid,
    #[error("token generation failed")]
    Generation,
}

pub trait TokenService: Send + Sync {
    fn create_access_token(&self, user_id: &Uuid) -> Result<String, TokenError>;
    fn create_refresh_token(&self, user_id: &Uuid) -> Result<String, TokenError>;
    fn validate_access_token(&self, token: &str) -> Result<TokenIdentity, TokenError>;
    fn validate_refresh_token(&self, token: &str) -> Result<TokenIdentity, TokenError>;
}
