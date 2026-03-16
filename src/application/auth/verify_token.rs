use uuid::Uuid;
use crate::domain::errors::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::auth::jwt::Jwt;
use crate::auth::claims::Claims;

#[derive(Clone)]
pub struct VerifyTokenUseCase<R: UserRepository> {
    user_repo: R,
    jwt: Jwt,
}

impl<R: UserRepository> VerifyTokenUseCase<R> {
    pub fn new(user_repo: R, jwt: Jwt) -> Self {
        Self { user_repo, jwt }
    }

    pub async fn execute(&self, token: String) -> Result<Claims, DomainError> {
        let claims = self.jwt.decode_token(&token)
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

        Ok(claims)
    }
}