use crate::application::{
    errors::ApplicationError,
    security::token_service::{TokenIdentity, TokenService},
};
use crate::domain::{errors::DomainError, user::repository::UserRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct VerifyTokenUseCase {
    users: Arc<dyn UserRepository>,
    tokens: Arc<dyn TokenService>,
}
impl VerifyTokenUseCase {
    pub fn new(users: Arc<dyn UserRepository>, tokens: Arc<dyn TokenService>) -> Self {
        Self { users, tokens }
    }
    pub async fn execute(&self, token: &str) -> Result<TokenIdentity, ApplicationError> {
        let identity = self.tokens.validate_access_token(token)?;
        if !self.users.exists(&identity.user_id).await? {
            return Err(DomainError::UserNotFound.into());
        }
        Ok(identity)
    }
}
