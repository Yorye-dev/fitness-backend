use crate::domain::errors::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::auth::jwt::Jwt;
use crate::dtos::sign_data_dto::SignInData;
use crate::auth;

pub struct LoginUseCase<R: UserRepository> {
    user_repo: R,
    jwt: Jwt,
}

impl<R: UserRepository> LoginUseCase<R> {
    pub fn new(user_repo: R, jwt: Jwt) -> Self {
        Self { user_repo, jwt }
    }

    pub async fn execute(&self, dto: SignInData) -> Result<String, DomainError> {
        let user = self.user_repo
            .get_sign_in_user_by_username(&dto.username)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        if !auth::utils::verify_password(&dto.password, &user.password_hash) {
            return Err(DomainError::InvalidCredentials);
        }

        let token = self.jwt.generate_token(&user.id, 60)
            .map_err(|_| DomainError::GenerateTokenError)?;

        Ok(token)
    }
}
