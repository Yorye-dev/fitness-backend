use crate::repositories::user_repository::UserRepository;
use crate::factories::user_factory::UserFactory;
use crate::dtos::login_user_dto::LoginUserDto;

#[derive(Clone)]
pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {

    pub fn new(repo: UserRepository) -> Self {
        
        Self { repo }
    }

    pub async fn login_user(&self, dto :LoginUserDto) {

        let user = self.repo.get_public_user_by_username(&dto.username).await;


    }
}
