use crate::models::user::{PublicUser};
use crate::repositories::user_repository::UserRepository;
use crate::dtos::login_user_dto::LoginUserDto;
use crate::errors::AuthError;
use crate::utils::password_utils;

#[derive(Clone)]
pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {

    pub fn new(repo: UserRepository) -> Self {
        
        Self { repo }
    }

    pub async fn login_user(&self, dto :LoginUserDto) -> Result<PublicUser, AuthError> {

        let user = self.repo
        .get_user_by_username(&dto.username)
        .await
        .map_err(AuthError::DatabaseError)?
        .ok_or(AuthError::UserNotFound)?;

        if !password_utils::verify_password(&dto.password,&user.password_hash){

            return Err(AuthError::InvalidCredentials)
        }

        Ok(user.into())
    }
}
