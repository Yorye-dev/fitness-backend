use crate::domain::nutrition::{
    consumption::DailyConsumption,
    goals::NutritionGoals,
    meal::{Meal, NutritionBasis},
    repository::{ConsumptionWithMeal, StatsSummary},
};
use crate::infrastructure::persistence::models::nutrition_row::*;

impl From<NutritionGoalsRow> for NutritionGoals {
    fn from(row: NutritionGoalsRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            protein_goal: row.protein_goal,
            fats_goal: row.fats_goal,
            carbs_goal: row.carbs_goal,
            tdee: row.tdee,
            bmr: row.bmr,
        }
    }
}
impl From<MealRow> for Meal {
    fn from(row: MealRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            nutrition_basis: if row.per_unit {
                NutritionBasis::PerUnit
            } else {
                NutritionBasis::Per100g
            },
            calories: row.calories,
            protein: row.protein,
            carbs: row.carbs,
            fat: row.fat,
        }
    }
}
impl From<ConsumptionRow> for DailyConsumption {
    fn from(row: ConsumptionRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            date: row.date,
            meal_id: row.meal_id,
            quantity_grams: row.quantity_grams,
            portion_count: row.portion_count,
            portion_grams: row.portion_grams,
            calories_consumed: row.calories_consumed,
            protein_consumed: row.protein_consumed,
            carbs_consumed: row.carbs_consumed,
            fat_consumed: row.fat_consumed,
        }
    }
}
impl From<ConsumptionWithMealRow> for ConsumptionWithMeal {
    fn from(row: ConsumptionWithMealRow) -> Self {
        Self {
            consumption: row.consumption.into(),
            meal_name: row.meal_name,
            meal_calories: row.meal_calories,
            meal_protein: row.meal_protein,
            meal_carbs: row.meal_carbs,
            meal_fat: row.meal_fat,
        }
    }
}
impl From<StatsRow> for StatsSummary {
    fn from(row: StatsRow) -> Self {
        let days = row.total_days.max(1) as f32;
        Self {
            total_days: row.total_days,
            total_calories: row.total_calories,
            total_protein: row.total_protein,
            total_carbs: row.total_carbs,
            total_fat: row.total_fat,
            avg_calories: row.total_calories / days,
            avg_protein: row.total_protein / days,
            avg_carbs: row.total_carbs / days,
            avg_fat: row.total_fat / days,
        }
    }
}
