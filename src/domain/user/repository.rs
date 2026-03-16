use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::user::{User, SignInUser};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save_user(&self, user: &User) -> Result<User, sqlx::Error>;
    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn get_user_by_username(&self, username: &String) -> Result<Option<User>, sqlx::Error>;
    async fn get_sign_in_user_by_username(&self, username: &String) -> Result<Option<SignInUser>, sqlx::Error>;
    async fn exists(&self, user_id: &Uuid) -> Result<bool, sqlx::Error>;
    async fn update_password(&self, user_id: &Uuid, new_password_hash: &str) -> Result<bool, sqlx::Error>;
}