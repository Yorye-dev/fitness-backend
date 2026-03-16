use uuid::Uuid;
use crate::domain::errors::DomainError;
use crate::domain::nutrition::repository::NutritionRepository;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::calculator::NutritionCalculator;

#[derive(Clone)]
pub struct UpdateGoalsUseCase<R: NutritionRepository> {
    nutrition_repo: R,
}

impl<R: NutritionRepository> UpdateGoalsUseCase<R> {
    pub fn new(nutrition_repo: R) -> Self {
        Self { nutrition_repo }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        weight: f32,
        height: f32,
        age: u32,
        sex: crate::domain::enums::sex::Sex,
        activity_level: crate::domain::enums::activity_level::ActivityLevel,
        goal: crate::domain::enums::goals::Goal,
    ) -> Result<NutritionGoals, DomainError> {
        let bmr = NutritionCalculator::calculate_bmr(weight, height, age, sex.clone());
        let tdee = NutritionCalculator::calculate_tdee(bmr, activity_level.clone());
        let macros = NutritionCalculator::calculate_macros(tdee, goal);

        let existing_goals = self.nutrition_repo.get_user_goals(&user_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e))?;

        let goals = if let Some(ref existing) = existing_goals {
            NutritionGoals::from_db(
                existing.id,
                user_id,
                macros.protein,
                macros.fat,
                macros.carbs,
                tdee,
                bmr
            )
        } else {
            NutritionGoals::new(
                user_id,
                macros.protein,
                macros.fat,
                macros.carbs,
                tdee,
                bmr
            )
        };

        let saved_goals = if existing_goals.is_some() {
            self.nutrition_repo.update_user_goals(&goals)
                .await
                .map_err(|e| DomainError::DatabaseError(e))?
        } else {
            self.nutrition_repo.save_user_goals(&goals)
                .await
                .map_err(|e| DomainError::DatabaseError(e))?
        };

        Ok(saved_goals)
    }
}