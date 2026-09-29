use super::create_meal::MealInput;
use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::DomainError,
    nutrition::{meal::Meal, repository::MealRepository},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdateMealUseCase {
    meals: Arc<dyn MealRepository>,
}
impl UpdateMealUseCase {
    pub fn new(meals: Arc<dyn MealRepository>) -> Self {
        Self { meals }
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        meal_id: Uuid,
        input: MealInput,
    ) -> Result<Meal, ApplicationError> {
        self.meals
            .update_meal(&input.into_meal(meal_id, user_id)?)
            .await?
            .ok_or_else(|| DomainError::MealNotFound.into())
    }
}
