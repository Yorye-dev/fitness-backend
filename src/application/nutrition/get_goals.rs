use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::DomainError,
    nutrition::{
        calculator::NutritionCalculator, goals::NutritionGoals, repository::NutritionRepository,
    },
    user::repository::UserRepository,
};
use chrono::NaiveDate;
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
        self.calculate_fallback(user_id).await
    }

    pub async fn execute_on_date(
        &self,
        user_id: Uuid,
        date: NaiveDate,
    ) -> Result<NutritionGoals, ApplicationError> {
        if let Some(goals) = self
            .nutrition
            .get_user_goals_on_date(&user_id, &date)
            .await?
        {
            return Ok(goals);
        }
        // Preserve the existing API's estimated-target fallback when no historical version exists.
        self.calculate_fallback(user_id).await
    }

    async fn calculate_fallback(&self, user_id: Uuid) -> Result<NutritionGoals, ApplicationError> {
        let user = self
            .users
            .get_user_by_id(&user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;
        Ok(NutritionCalculator::goals_for(&user))
    }
}
