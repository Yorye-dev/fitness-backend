use crate::application::nutrition::create_meal::MealInput;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MealRequest {
    pub name: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fat_per_100g: f32,
}
impl From<MealRequest> for MealInput {
    fn from(request: MealRequest) -> Self {
        Self {
            name: request.name,
            calories_per_100g: request.calories_per_100g,
            protein_per_100g: request.protein_per_100g,
            carbs_per_100g: request.carbs_per_100g,
            fat_per_100g: request.fat_per_100g,
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MealListQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}
fn default_page() -> u32 {
    1
}
fn default_per_page() -> u32 {
    20
}
