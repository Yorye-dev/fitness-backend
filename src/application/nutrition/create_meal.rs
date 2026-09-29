use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::DomainError,
    nutrition::{meal::Meal, repository::MealRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct MealInput {
    pub name: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fat_per_100g: f32,
}
impl MealInput {
    pub fn into_meal(self, id: Uuid, user_id: Uuid) -> Result<Meal, DomainError> {
        let name = self.name.trim();
        if name.is_empty()
            || name.chars().count() > 200
            || [
                self.calories_per_100g,
                self.protein_per_100g,
                self.carbs_per_100g,
                self.fat_per_100g,
            ]
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
            || [
                self.protein_per_100g,
                self.carbs_per_100g,
                self.fat_per_100g,
            ]
            .iter()
            .any(|value| *value > 100.0)
            || self.calories_per_100g >= 100_000.0
        {
            return Err(DomainError::Validation(
                "invalid meal name or nutritional values".into(),
            ));
        }
        Ok(Meal {
            id,
            user_id,
            name: name.to_owned(),
            calories_per_100g: self.calories_per_100g,
            protein_per_100g: self.protein_per_100g,
            carbs_per_100g: self.carbs_per_100g,
            fat_per_100g: self.fat_per_100g,
        })
    }
}
#[derive(Clone)]
pub struct CreateMealUseCase {
    meals: Arc<dyn MealRepository>,
}
impl CreateMealUseCase {
    pub fn new(meals: Arc<dyn MealRepository>) -> Self {
        Self { meals }
    }
    pub async fn execute(&self, user_id: Uuid, input: MealInput) -> Result<Meal, ApplicationError> {
        Ok(self
            .meals
            .save_meal(&input.into_meal(Uuid::new_v4(), user_id)?)
            .await?)
    }
}
