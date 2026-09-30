use crate::domain::nutrition::meal::{Meal, NutritionBasis};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct MealResponse {
    pub id: Uuid,
    pub name: String,
    pub nutrition_basis: &'static str,
    pub calories_per_100g: Option<f32>,
    pub protein_per_100g: Option<f32>,
    pub carbs_per_100g: Option<f32>,
    pub fat_per_100g: Option<f32>,
    pub calories_per_unit: Option<f32>,
    pub protein_per_unit: Option<f32>,
    pub carbs_per_unit: Option<f32>,
    pub fat_per_unit: Option<f32>,
}
impl From<Meal> for MealResponse {
    fn from(meal: Meal) -> Self {
        let weight = meal.nutrition_basis == NutritionBasis::Per100g;
        Self {
            id: meal.id,
            name: meal.name,
            nutrition_basis: meal.nutrition_basis.as_str(),
            calories_per_100g: weight.then_some(meal.calories),
            protein_per_100g: weight.then_some(meal.protein),
            carbs_per_100g: weight.then_some(meal.carbs),
            fat_per_100g: weight.then_some(meal.fat),
            calories_per_unit: (!weight).then_some(meal.calories),
            protein_per_unit: (!weight).then_some(meal.protein),
            carbs_per_unit: (!weight).then_some(meal.carbs),
            fat_per_unit: (!weight).then_some(meal.fat),
        }
    }
}
