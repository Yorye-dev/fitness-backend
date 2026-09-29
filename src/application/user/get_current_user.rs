use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::DomainError,
    user::{entity::User, repository::UserRepository},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetCurrentUserUseCase {
    users: Arc<dyn UserRepository>,
}
impl GetCurrentUserUseCase {
    pub fn new(users: Arc<dyn UserRepository>) -> Self {
        Self { users }
    }
    pub async fn execute(&self, user_id: Uuid) -> Result<User, ApplicationError> {
        self.users
            .get_user_by_id(&user_id)
            .await?
            .ok_or_else(|| DomainError::UserNotFound.into())
    }
}
