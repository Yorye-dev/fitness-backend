use crate::domain::user::entity::{SignInUser, User};
use crate::domain::{errors::RepositoryError, nutrition::goals::NutritionGoals};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save_user(&self, user: &User, goals: &NutritionGoals)
    -> Result<User, RepositoryError>;
    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, RepositoryError>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError>;
    async fn get_sign_in_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<SignInUser>, RepositoryError>;
    async fn exists(&self, user_id: &Uuid) -> Result<bool, RepositoryError>;
    async fn update_password(
        &self,
        user_id: &Uuid,
        new_password_hash: &str,
    ) -> Result<bool, RepositoryError>;
    async fn update_user(
        &self,
        user: &User,
        goals: &NutritionGoals,
    ) -> Result<User, RepositoryError>;
}
