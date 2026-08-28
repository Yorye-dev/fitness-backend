use crate::domain::errors::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user::PublicUser;
use crate::domain::nutrition::repository::NutritionRepository;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::calculator::NutritionCalculator;
use crate::domain::enums::{activity_level::ActivityLevel, goals::Goal};
use crate::dtos::update_user_dto::UpdateUserDto;
use crate::dtos::response_user_nutrition_goals_dto::ResponseUserNutritionGoalsDto;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdateUserUseCase<R: UserRepository, N: NutritionRepository> {
    user_repo: R,
    nutrition_repo: N,
}

impl<R: UserRepository, N: NutritionRepository> UpdateUserUseCase<R, N> {
    pub fn new(user_repo: R, nutrition_repo: N) -> Self {
        Self { user_repo, nutrition_repo }
    }

    pub async fn execute(&self, user_id: Uuid, dto: UpdateUserDto) -> Result<(PublicUser, ResponseUserNutritionGoalsDto), DomainError> {
        let mut user = self.user_repo.get_user_by_id(&user_id)
            .await
            .map_err(|_| DomainError::UserNotFound)?
            .ok_or(DomainError::UserNotFound)?;

        let activity_enum = match dto.activity_level.to_lowercase().as_str() {
            "sedentary" => ActivityLevel::Sedentary,
            "lightly_active" => ActivityLevel::LightlyActive,
            "moderately_active" => ActivityLevel::ModeratelyActive,
            "very_active" => ActivityLevel::VeryActive,
            "extra_active" => ActivityLevel::ExtraActive,
            _ => return Err(DomainError::ValidationError("Invalid activity_level".to_string())),
        };

        let goal_enum = match dto.goal.to_lowercase().as_str() {
            "lose_weight" => Goal::LoseWeight,
            "maintain" => Goal::Maintain,
            "gain_muscle" => Goal::GainMuscle,
            _ => return Err(DomainError::ValidationError("Invalid goal".to_string())),
        };

        user.weight = dto.weight;
        user.height = dto.height;
        user.age = dto.age;
        user.activity_level = activity_enum;
        user.goal = goal_enum;

        let updated_user = self.user_repo.update_user(&user).await?;

        let goals = Self::recalculate_goals(&updated_user);
        self.nutrition_repo.update_user_goals(&goals).await?;

        let goals_dto = ResponseUserNutritionGoalsDto::from(goals);

        Ok((PublicUser::from(updated_user), goals_dto))
    }

    fn recalculate_goals(user: &crate::domain::user::user::User) -> NutritionGoals {
        let bmr = NutritionCalculator::calculate_bmr(
            user.weight, 
            user.height as f32, 
            user.age as u32, 
            user.sex.clone()
        );
        let tdee = NutritionCalculator::calculate_tdee(bmr, user.activity_level.clone());
        let macros = NutritionCalculator::calculate_macros(tdee, user.goal.clone());

        NutritionGoals::new(
            user.id,
            macros.protein,
            macros.fat,
            macros.carbs,
            tdee,
            bmr
        )
    }
}