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

pub struct UpdateGoalsInput {
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
}
#[derive(Clone)]
pub struct UpdateGoalsUseCase {
    nutrition: Arc<dyn NutritionRepository>,
    users: Arc<dyn UserRepository>,
}
impl UpdateGoalsUseCase {
    pub fn new(nutrition: Arc<dyn NutritionRepository>, users: Arc<dyn UserRepository>) -> Self {
        Self { nutrition, users }
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        input: UpdateGoalsInput,
    ) -> Result<NutritionGoals, ApplicationError> {
        if [
            input.protein_goal,
            input.carbs_goal,
            input.fats_goal,
            input.tdee,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v < 0.0 || *v >= 10_000_000.0)
            || input.tdee == 0.0
        {
            return Err(DomainError::Validation(
                "nutrition targets must be finite and non-negative, with positive calories".into(),
            )
            .into());
        }
        let user = self
            .users
            .get_user_by_id(&user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;
        let mut goals = NutritionCalculator::goals_for(&user);
        goals.protein_goal = input.protein_goal;
        goals.fats_goal = input.fats_goal;
        goals.carbs_goal = input.carbs_goal;
        goals.tdee = input.tdee;
        Ok(self.nutrition.save_user_goals(&goals).await?)
    }
}
