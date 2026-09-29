use crate::application::errors::ApplicationError;
use crate::domain::{
    errors::DomainError,
    nutrition::{meal::Meal, repository::MealRepository},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetMealUseCase {
    meals: Arc<dyn MealRepository>,
}
impl GetMealUseCase {
    pub fn new(meals: Arc<dyn MealRepository>) -> Self {
        Self { meals }
    }
    pub async fn execute(&self, user_id: Uuid, meal_id: Uuid) -> Result<Meal, ApplicationError> {
        self.meals
            .get_meal_by_id(&meal_id, &user_id)
            .await?
            .ok_or_else(|| DomainError::MealNotFound.into())
    }
}
#[derive(Clone)]
pub struct ListMealsUseCase {
    meals: Arc<dyn MealRepository>,
}
impl ListMealsUseCase {
    pub fn new(meals: Arc<dyn MealRepository>) -> Self {
        Self { meals }
    }
    pub async fn execute(
        &self,
        user_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Meal>, i64), ApplicationError> {
        if page == 0 || !(1..=100).contains(&per_page) {
            return Err(DomainError::Validation(
                "page must be positive and per_page between 1 and 100".into(),
            )
            .into());
        }
        Ok(self
            .meals
            .get_meals_paginated(&user_id, page, per_page)
            .await?)
    }
}
