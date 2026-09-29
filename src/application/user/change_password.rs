use crate::application::{errors::ApplicationError, security::password_service::PasswordService};
use crate::domain::{errors::DomainError, user::repository::UserRepository};
use std::sync::Arc;
use uuid::Uuid;

pub(super) fn validate_new_password(password: &str) -> Result<(), DomainError> {
    if !(8..=1024).contains(&password.len()) {
        return Err(DomainError::Validation(
            "password must be between 8 and 1024 bytes".into(),
        ));
    }
    Ok(())
}
#[derive(Clone)]
pub struct ChangePasswordUseCase {
    users: Arc<dyn UserRepository>,
    passwords: Arc<dyn PasswordService>,
}
impl ChangePasswordUseCase {
    pub fn new(users: Arc<dyn UserRepository>, passwords: Arc<dyn PasswordService>) -> Self {
        Self { users, passwords }
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        current: &str,
        new_password: &str,
    ) -> Result<(), ApplicationError> {
        validate_new_password(new_password)?;
        let user = self
            .users
            .get_user_by_id(&user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;
        if !self.passwords.verify(current, user.password_hash()).await? {
            return Err(DomainError::InvalidCredentials.into());
        }
        let hash = self.passwords.hash(new_password).await?;
        if !self.users.update_password(&user_id, &hash).await? {
            return Err(DomainError::UserNotFound.into());
        }
        Ok(())
    }
}
