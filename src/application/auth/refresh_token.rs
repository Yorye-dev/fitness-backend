use crate::application::{errors::ApplicationError, security::token_service::TokenService};
use crate::domain::{errors::DomainError, user::repository::UserRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct RefreshTokenUseCase {
    users: Arc<dyn UserRepository>,
    tokens: Arc<dyn TokenService>,
}
impl RefreshTokenUseCase {
    pub fn new(users: Arc<dyn UserRepository>, tokens: Arc<dyn TokenService>) -> Self {
        Self { users, tokens }
    }
    pub async fn execute(&self, token: &str) -> Result<String, ApplicationError> {
        let identity = self.tokens.validate_refresh_token(token)?;
        if !self.users.exists(&identity.user_id).await? {
            return Err(DomainError::UserNotFound.into());
        }
        Ok(self.tokens.create_access_token(&identity.user_id)?)
    }
}
