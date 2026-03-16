use crate::domain::errors::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::auth::jwt::Jwt;
use uuid::Uuid;

#[derive(Clone)]
pub struct RefreshTokenUseCase<R: UserRepository> {
    user_repo: R,
    jwt: Jwt,
}

impl<R: UserRepository> RefreshTokenUseCase<R> {
    pub fn new(user_repo: R, jwt: Jwt) -> Self {
        Self { user_repo, jwt }
    }

    pub async fn execute(&self, refresh_token: String) -> Result<String, DomainError> {
        let claims = self.jwt.decode_token(&refresh_token)
            .map_err(|_| DomainError::InvalidToken)?;

        let user_id = claims.subject.clone();
        
        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|_| DomainError::InvalidToken)?;
        
        let exists = self.user_repo.exists(&user_uuid)
            .await
            .map_err(|e| DomainError::DatabaseError(e))?;

        if !exists {
            return Err(DomainError::UserNotFound);
        }

        let new_access_token = self.jwt.generate_access_token(&user_uuid)
            .map_err(|_| DomainError::GenerateTokenError)?;

        Ok(new_access_token)
    }
}
