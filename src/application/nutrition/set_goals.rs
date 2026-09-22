use uuid::Uuid;

use crate::domain::errors::DomainError;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::repository::NutritionRepository;

#[derive(Debug)]
pub struct SetGoalsInput {
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
    pub bmr: f32,
}

pub struct SetGoalsUseCase<R: NutritionRepository> {
    nutrition_repo: R,
}

impl<R: NutritionRepository> SetGoalsUseCase<R> {
    pub fn new(nutrition_repo: R) -> Self {
        Self { nutrition_repo }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        input: SetGoalsInput,
    ) -> Result<NutritionGoals, DomainError> {
        let goals = NutritionGoals::new(
            user_id,
            input.protein_goal,
            input.fats_goal,
            input.carbs_goal,
            input.tdee,
            input.bmr,
        );

        self.nutrition_repo.save_user_goals(&goals).await?;

        Ok(goals)
    }
}
