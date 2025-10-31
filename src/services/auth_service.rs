use std::primitive;

use crate::auth::claims::Claims;
use crate::dtos::register_user_dto::RegisterUserDto;
use crate::repositories::user_repository::UserRepository;
use crate::errors::AuthError;
use crate::factories::user_factory::UserFactory;
use crate::auth;
use crate::dtos::sign_data_dto::SignInData;
use crate::auth::jwt::Jwt;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    jwt: Jwt,
}

impl AuthService {

    pub fn new (user_repo: UserRepository, jwt: Jwt ) -> Self {
        
        Self { user_repo, jwt }
    }

    pub async fn sing_in (&self, dto :SignInData) -> Result<String, AuthError> {
        
        let user = self.user_repo
            .get_sign_in_user_by_username(&dto.username)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        if !auth::utils::verify_password(&dto.password, &user.password_hash){

            return Err(AuthError::InvalidCredentials);
        }

        let token_data = self.jwt.generate_token(&user.id, 60)
            .map_err(|_| AuthError::GenerateTokenError);

        Ok(token_data?)
    }

    pub async fn register(&self, dto :RegisterUserDto) -> Result<String, AuthError> {

        let user = UserFactory::create_user_from_dto(dto).unwrap();

        self.user_repo.save_user(&user);//Propagar el error desde los repos.

        let token_data = self.jwt.generate_token(&user.id.to_string(), 60)
            .map_err(|_| AuthError::GenerateTokenError);

        Ok(token_data?)        
    }

    pub async fn get_claims_if_valid(&self, token :String) -> Result<Claims, AuthError> {

        let claims  = self.jwt.decode_token(&token).map_err(|_| AuthError::GenerateTokenError)?;

        let user_id = claims.subject.clone();

        let exists = self.user_repo.exists(&user_id)
            .await
            .map_err(|_| AuthError::UserNotFound)?;

        if !exists {
            return Err(AuthError::UserNotFound)
        }

        Ok(claims)
    }

     async fn user_exists(&self, user_id: &String) -> Result<bool, AuthError> {

        self.user_repo.exists(user_id).await.map_err(|_| AuthError::UserNotFound)
    }
}
