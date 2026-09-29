use crate::application::errors::ApplicationError;
use crate::domain::{errors::DomainError, nutrition::repository::MealRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeleteMealUseCase {
    meals: Arc<dyn MealRepository>,
}
impl DeleteMealUseCase {
    pub fn new(meals: Arc<dyn MealRepository>) -> Self {
        Self { meals }
    }
    pub async fn execute(&self, user_id: Uuid, meal_id: Uuid) -> Result<(), ApplicationError> {
        if !self.meals.delete_meal(&meal_id, &user_id).await? {
            return Err(DomainError::MealNotFound.into());
        }
        Ok(())
    }
}
