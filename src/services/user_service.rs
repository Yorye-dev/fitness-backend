use crate::repositories::user_repository::UserRepository;
use crate::factories::user_factory::UserFactory;
use crate::dtos::register_user_dto::RegisterUserDto;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {

    pub fn new(repo: UserRepository) -> Self {
        
        Self { repo }
    }

    pub async fn register_user(&self, dto : RegisterUserDto) {

        let user = UserFactory::create_user_from_dto(dto).unwrap();

        let _ =self.repo.save_user(&user).await;
        
    }
}
