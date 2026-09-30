use crate::application::nutrition::create_meal::MealInput;
use crate::{domain::nutrition::meal::NutritionBasis, presentation::errors::api_error::ApiError};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MealRequest {
    pub name: String,
    pub nutrition_basis: Option<String>,
    pub calories_per_100g: Option<f32>,
    pub protein_per_100g: Option<f32>,
    pub carbs_per_100g: Option<f32>,
    pub fat_per_100g: Option<f32>,
    pub calories_per_unit: Option<f32>,
    pub protein_per_unit: Option<f32>,
    pub carbs_per_unit: Option<f32>,
    pub fat_per_unit: Option<f32>,
}
impl TryFrom<MealRequest> for MealInput {
    type Error = ApiError;
    fn try_from(request: MealRequest) -> Result<Self, Self::Error> {
        let weight = [
            request.calories_per_100g,
            request.protein_per_100g,
            request.carbs_per_100g,
            request.fat_per_100g,
        ];
        let unit = [
            request.calories_per_unit,
            request.protein_per_unit,
            request.carbs_per_unit,
            request.fat_per_unit,
        ];
        let (basis, values, unused) = match request.nutrition_basis.as_deref().unwrap_or("per_100g")
        {
            "per_100g" => (NutritionBasis::Per100g, weight, unit),
            "per_unit" => (NutritionBasis::PerUnit, unit, weight),
            _ => {
                return Err(ApiError::Validation(
                    "nutrition_basis must be per_100g or per_unit".into(),
                ));
            }
        };
        if unused.iter().any(Option::is_some) {
            return Err(ApiError::Validation(
                "provide nutrients only for the selected basis".into(),
            ));
        }
        let [Some(calories), Some(protein), Some(carbs), Some(fat)] = values else {
            return Err(ApiError::Validation(
                "provide all four nutrients for the selected basis".into(),
            ));
        };
        Ok(Self {
            name: request.name,
            nutrition_basis: basis,
            calories,
            protein,
            carbs,
            fat,
        })
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MealListQuery {
    pub q: Option<String>,
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
