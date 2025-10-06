use crate::models::user::{PublicUser};
use crate::repositories::user_repository::UserRepository;
use crate::dtos::login_user_dto::LoginUserDto;
use crate::errors::AuthError;
use crate::utils::password_utils;
use crate::factories::api_response_factory;
use crate::auth;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
}

impl AuthService {

    pub fn new ( user_repo: UserRepository ) -> Self {
        
        Self { user_repo }
    }

    pub async fn login_user(&self, dto :LoginUserDto) -> Result<PublicUser, AuthError> {

        let user = self.user_repo
        .get_user_by_username(&dto.username)
        .await
        .map_err(AuthError::DatabaseError)?
        .ok_or(AuthError::UserNotFound)?;

        if !password_utils::verify_password(&dto.password,&user.password_hash){

            return Err(AuthError::InvalidCredentials)
        }

        Ok(user.into())
    }

    pub async fn sing_in (&self, dto :LoginUserDto) -> Result<String, AuthError> {
        
        let user = self.user_repo
            .get_sign_in_user_by_username(&dto.username)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        if !auth::utils::verify_password(&dto.password, &user.password_hash){

            return Err(AuthError::InvalidCredentials);
        }

        let token_data = auth::jwt::generate_token(&user.id)
            .map_err(|_| AuthError::GenerateTokenError);

        Ok(token_data?)
    }
}
