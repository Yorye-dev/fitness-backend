use uuid::Uuid;

use crate::domain::enums::{activity_level::ActivityLevel, goals::Goal};
use crate::domain::errors::DomainError;
use crate::domain::nutrition::calculator::NutritionCalculator;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::repository::NutritionRepository;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user::{PublicUser, User};

#[derive(Debug)]
pub struct UpdateUserInput {
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: ActivityLevel,
    pub goal: Goal,
}

#[derive(Clone)]
pub struct UpdateUserUseCase<R: UserRepository, N: NutritionRepository> {
    user_repo: R,
    nutrition_repo: N,
}

impl<R: UserRepository, N: NutritionRepository> UpdateUserUseCase<R, N> {
    pub fn new(user_repo: R, nutrition_repo: N) -> Self {
        Self {
            user_repo,
            nutrition_repo,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        input: UpdateUserInput,
    ) -> Result<(PublicUser, NutritionGoals), DomainError> {
        let mut user = self
            .user_repo
            .get_user_by_id(&user_id)
            .await
            .map_err(|_| DomainError::UserNotFound)?
            .ok_or(DomainError::UserNotFound)?;

        user.weight = input.weight;
        user.height = input.height;
        user.age = input.age;
        user.activity_level = input.activity_level;
        user.goal = input.goal;

        let updated_user = self.user_repo.update_user(&user).await?;

        let goals = Self::recalculate_goals(&updated_user);

        let updated_goals = self.nutrition_repo.update_user_goals(&goals).await?;

        Ok((PublicUser::from(updated_user), updated_goals))
    }

    fn recalculate_goals(user: &User) -> NutritionGoals {
        let bmr = NutritionCalculator::calculate_bmr(
            user.weight,
            user.height as f32,
            user.age as u32,
            user.sex.clone(),
        );

        let tdee = NutritionCalculator::calculate_tdee(bmr, user.activity_level.clone());

        let macros = NutritionCalculator::calculate_macros(tdee, user.goal.clone());

        NutritionGoals::new(user.id, macros.protein, macros.fat, macros.carbs, tdee, bmr)
    }
}
