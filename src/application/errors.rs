use super::security::{password_service::PasswordError, token_service::TokenError};
use crate::domain::errors::{DomainError, RepositoryError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error(transparent)]
    Token(#[from] TokenError),
    #[error(transparent)]
    Password(#[from] PasswordError),
}
