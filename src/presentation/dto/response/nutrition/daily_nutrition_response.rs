use crate::domain::nutrition::{daily::DailyNutrition, repository::ConsumptionWithMeal};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct NutrientResponse {
    pub consumed: f32,
    pub target: f32,
}
#[derive(Serialize)]
pub struct DailyMacrosResponse {
    pub protein: NutrientResponse,
    pub carbs: NutrientResponse,
    pub fat: NutrientResponse,
}
#[derive(Serialize)]
pub struct DailyMealResponse {
    pub id: Uuid,
    pub meal_id: Uuid,
    pub name: String,
    pub quantity_grams: Option<f64>,
    pub portion_count: Option<f64>,
    pub portion_grams: Option<f64>,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}
impl From<ConsumptionWithMeal> for DailyMealResponse {
    fn from(meal: ConsumptionWithMeal) -> Self {
        let c = meal.consumption;
        Self {
            id: c.id,
            meal_id: c.meal_id,
            name: meal.meal_name,
            quantity_grams: c.quantity_grams,
            portion_count: c.portion_count,
            portion_grams: c.portion_grams,
            calories: c.calories_consumed,
            protein: c.protein_consumed,
            carbs: c.carbs_consumed,
            fat: c.fat_consumed,
        }
    }
}
#[derive(Serialize)]
pub struct DailyNutritionResponse {
    pub date: String,
    pub calories: NutrientResponse,
    pub macros: DailyMacrosResponse,
    pub meals: Vec<DailyMealResponse>,
}
impl From<DailyNutrition> for DailyNutritionResponse {
    fn from(day: DailyNutrition) -> Self {
        Self {
            date: day.date.format("%Y-%m-%d").to_string(),
            calories: NutrientResponse {
                consumed: day.consumed.calories,
                target: day.goals.tdee,
            },
            macros: DailyMacrosResponse {
                protein: NutrientResponse {
                    consumed: day.consumed.protein,
                    target: day.goals.protein_goal,
                },
                carbs: NutrientResponse {
                    consumed: day.consumed.carbs,
                    target: day.goals.carbs_goal,
                },
                fat: NutrientResponse {
                    consumed: day.consumed.fat,
                    target: day.goals.fats_goal,
                },
            },
            meals: day.meals.into_iter().map(Into::into).collect(),
        }
    }
}
