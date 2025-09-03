use std::sync::Arc;

use repositories::user_repository::UserRepository;

pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<UserRepo>) -> Self {
        Self { repo }
    }

    pub async fn create_user(dto: &userDto) -> Result<User, String> {
        let user = User::new(username.to_string(), password.to_string(), age);
        self.repo.create_user(&user).await.map_err(|e| e.to_string())
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        self.repo.get_user_by_username(username).await.map_err(|e| e.to_string())
    }
}:
