use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::DomainError,
    nutrition::calculator::NutritionCalculator,
    user::{
        activity_level::ActivityLevel, entity::User, goal::Goal, profile::UserProfile,
        repository::UserRepository,
    },
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct UpdateUserInput {
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: ActivityLevel,
    pub goal: Goal,
}
#[derive(Clone)]
pub struct UpdateUserUseCase {
    users: Arc<dyn UserRepository>,
}
impl UpdateUserUseCase {
    pub fn new(users: Arc<dyn UserRepository>) -> Self {
        Self { users }
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        input: UpdateUserInput,
    ) -> Result<User, ApplicationError> {
        let profile = UserProfile::new(
            input.weight,
            input.height,
            input.age,
            input.activity_level,
            input.goal,
        )?;
        let mut user = self
            .users
            .get_user_by_id(&user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;
        user.update_profile(profile);
        let goals = NutritionCalculator::goals_for(&user);
        Ok(self.users.update_user(&user, &goals).await?)
    }
}
