use uuid::Uuid;
use crate::domain::errors::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::auth;

#[derive(Clone)]
pub struct ChangePasswordUseCase<R: UserRepository> {
    user_repo: R,
}

impl<R: UserRepository> ChangePasswordUseCase<R> {
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user_id: Uuid, current_password: String, new_password: String) -> Result<bool, DomainError> {
        let user = self.user_repo.get_user_by_id(&user_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e))?
            .ok_or(DomainError::UserNotFound)?;

        if !auth::utils::verify_password(&current_password, &user.password_hash) {
            return Err(DomainError::InvalidCredentials);
        }

        let new_hash = crate::shared::password::calculate_hash(&new_password)
            .map_err(|_| DomainError::HashingError)?;

        let updated = self.user_repo.update_password(&user_id, &new_hash)
            .await
            .map_err(|e| DomainError::DatabaseError(e))?;

        Ok(updated)
    }
}