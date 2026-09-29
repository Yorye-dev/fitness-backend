use crate::application::{
    errors::ApplicationError,
    security::{password_service::PasswordService, token_service::TokenService},
};
use crate::domain::{errors::DomainError, user::repository::UserRepository};
use std::sync::Arc;

pub struct LoginInput {
    pub username: String,
    pub password: String,
}
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct LoginUseCase {
    users: Arc<dyn UserRepository>,
    tokens: Arc<dyn TokenService>,
    passwords: Arc<dyn PasswordService>,
}
impl LoginUseCase {
    pub fn new(
        users: Arc<dyn UserRepository>,
        tokens: Arc<dyn TokenService>,
        passwords: Arc<dyn PasswordService>,
    ) -> Self {
        Self {
            users,
            tokens,
            passwords,
        }
    }
    pub async fn execute(&self, input: LoginInput) -> Result<AuthTokens, ApplicationError> {
        let user = self
            .users
            .get_sign_in_user_by_username(&input.username)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;
        if !self
            .passwords
            .verify(&input.password, user.password_hash())
            .await?
        {
            return Err(DomainError::InvalidCredentials.into());
        }
        Ok(AuthTokens {
            access_token: self.tokens.create_access_token(&user.id())?,
            refresh_token: self.tokens.create_refresh_token(&user.id())?,
        })
    }
}
