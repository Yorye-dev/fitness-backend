use std::sync::Arc;

use repositories::user_repository::UserRepository;

pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<UserRepo>) -> Self {
        Self { repo }
    }

    pub async fn create_user(dto: &registerUserDto) -> Result<User, String> {
        // Agregar llamada a user factory, ya que se ve a encarga de recibir, los parametros y tranaformalos en un User
        let user = User::new(username.to_string(), password.to_string(), age);
        self.repo.create_user(&user).await.map_err(|e| e.to_string())
    }
}
