use super::{goals::NutritionGoals, macros::Macros, repository::ConsumptionWithMeal};
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct DailyNutrition {
    pub date: NaiveDate,
    pub consumed: Macros,
    pub goals: NutritionGoals,
    pub meals: Vec<ConsumptionWithMeal>,
}

impl DailyNutrition {
    pub fn new(date: NaiveDate, goals: NutritionGoals, meals: Vec<ConsumptionWithMeal>) -> Self {
        let mut consumed = Macros::default();
        for meal in &meals {
            consumed.calories += meal.consumption.calories_consumed;
            consumed.protein += meal.consumption.protein_consumed;
            consumed.carbs += meal.consumption.carbs_consumed;
            consumed.fat += meal.consumption.fat_consumed;
        }
        Self {
            date,
            consumed,
            goals,
            meals,
        }
    }
}
