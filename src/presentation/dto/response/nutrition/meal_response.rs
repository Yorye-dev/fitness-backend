use crate::domain::nutrition::meal::Meal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct MealResponse {
    pub id: Uuid,
    pub name: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fat_per_100g: f32,
}
impl From<Meal> for MealResponse {
    fn from(meal: Meal) -> Self {
        Self {
            id: meal.id,
            name: meal.name,
            calories_per_100g: meal.calories_per_100g,
            protein_per_100g: meal.protein_per_100g,
            carbs_per_100g: meal.carbs_per_100g,
            fat_per_100g: meal.fat_per_100g,
        }
    }
}
