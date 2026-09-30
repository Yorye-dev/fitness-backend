use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::DomainError,
    nutrition::{
        meal::{Meal, NutritionBasis},
        repository::MealRepository,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct MealInput {
    pub name: String,
    pub nutrition_basis: NutritionBasis,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}
impl MealInput {
    pub fn into_meal(self, id: Uuid, user_id: Uuid) -> Result<Meal, DomainError> {
        let name = self.name.trim();
        if name.is_empty()
            || name.chars().count() > 200
            || [self.calories, self.protein, self.carbs, self.fat]
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0 || *value >= 100_000.0)
            || [self.protein, self.carbs, self.fat]
                .iter()
                .any(|value| self.nutrition_basis == NutritionBasis::Per100g && *value > 100.0)
        {
            return Err(DomainError::Validation(
                "invalid meal name or nutritional values".into(),
            ));
        }
        Ok(Meal {
            id,
            user_id,
            name: name.to_owned(),
            nutrition_basis: self.nutrition_basis,
            calories: self.calories,
            protein: self.protein,
            carbs: self.carbs,
            fat: self.fat,
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
