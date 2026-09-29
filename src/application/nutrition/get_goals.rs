use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::DomainError,
    nutrition::{
        calculator::NutritionCalculator, goals::NutritionGoals, repository::NutritionRepository,
    },
    user::repository::UserRepository,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetGoalsUseCase {
    nutrition: Arc<dyn NutritionRepository>,
    users: Arc<dyn UserRepository>,
}
impl GetGoalsUseCase {
    pub fn new(nutrition: Arc<dyn NutritionRepository>, users: Arc<dyn UserRepository>) -> Self {
        Self { nutrition, users }
    }
    pub async fn execute(&self, user_id: Uuid) -> Result<NutritionGoals, ApplicationError> {
        if let Some(goals) = self.nutrition.get_user_goals(&user_id).await? {
            return Ok(goals);
        }
        let user = self
            .users
            .get_user_by_id(&user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;
        Ok(NutritionCalculator::goals_for(&user))
    }
}
