use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::errors::DomainError;
use crate::domain::nutrition::repository::NutritionRepository;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::dtos::create_user_nutrition_goals_dto::CreateUserNutritionsGoalsDto;

pub struct SetGoalsUseCase<R: NutritionRepository> {
    nutrition_repo: R,
}

impl<R: NutritionRepository> SetGoalsUseCase<R> {
    pub fn new(nutrition_repo: R) -> Self {
        Self { nutrition_repo }
    }

    pub async fn execute(&self, user_id: Uuid, dto: CreateUserNutritionsGoalsDto) -> Result<NutritionGoals, DomainError> {
        let goals = NutritionGoals::new(
            user_id,
            dto.protein_goal,
            dto.fats_goal,
            dto.carbs_goal,
            dto.tdee,
            dto.bmr,
        );

        self.nutrition_repo.save_user_goals(&goals).await?;

        Ok(goals)
    }
}
