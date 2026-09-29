use super::get_goals::GetGoalsUseCase;
use crate::application::errors::ApplicationError;
use crate::domain::nutrition::{daily::DailyNutrition, repository::NutritionRepository};
use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetDailyNutritionUseCase {
    nutrition: Arc<dyn NutritionRepository>,
    goals: GetGoalsUseCase,
}
impl GetDailyNutritionUseCase {
    pub fn new(nutrition: Arc<dyn NutritionRepository>, goals: GetGoalsUseCase) -> Self {
        Self { nutrition, goals }
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        date: NaiveDate,
    ) -> Result<DailyNutrition, ApplicationError> {
        let goals = self.goals.execute_on_date(user_id, date).await?;
        let meals = self.nutrition.get_daily_meals(&user_id, &date).await?;
        Ok(DailyNutrition::new(date, goals, meals))
    }
}
