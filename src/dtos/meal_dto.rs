use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateMealDto {
    pub name: String,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}

