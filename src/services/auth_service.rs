use crate::repositories::user_repository::UserRepository;
use crate::factories::user_factory::UserFactory;
use crate::dtos::login_user_dto::LoginUserDto;
use crate::errors::AuthError;

#[derive(Clone)]
pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {

    pub fn new(repo: UserRepository) -> Self {
        
        Self { repo }
    }

    pub async fn login_user(&self, dto :LoginUserDto) {

        let user = self.repo
        .get_user_by_username(&dto.username)
        .await
        .map_err(|_| AuthError::DatabaseError)
        .ok_or(AuthError::UserNotFound);
    }
}
