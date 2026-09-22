use serde::Serialize;

use crate::auth;
use crate::auth::jwt::Jwt;
use crate::domain::errors::DomainError;
use crate::domain::user::repository::UserRepository;

#[derive(Debug)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct LoginUseCase<R: UserRepository> {
    user_repo: R,
    jwt: Jwt,
}

impl<R: UserRepository> LoginUseCase<R> {
    pub fn new(user_repo: R, jwt: Jwt) -> Self {
        Self { user_repo, jwt }
    }

    pub async fn execute(&self, input: LoginInput) -> Result<AuthTokens, DomainError> {
        let user = self
            .user_repo
            .get_sign_in_user_by_username(&input.username)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        if !auth::utils::verify_password(&input.password, &user.password_hash) {
            return Err(DomainError::InvalidCredentials);
        }

        let access_token = self
            .jwt
            .generate_access_token(&user.id)
            .map_err(|_| DomainError::GenerateTokenError)?;

        let refresh_token = self
            .jwt
            .generate_refresh_token(&user.id)
            .map_err(|_| DomainError::GenerateTokenError)?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
        })
    }
}
