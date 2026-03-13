use crate::domain::nutrition::macros::Macros;
use crate::enums::{activity_level::ActivityLevel, goals::Goal, sex::Sex};

pub struct NutritionCalculator;

impl NutritionCalculator {
    pub fn calculate_bmr(weight_kg: f32, height_cm: f32, age: u32, sex: Sex) -> f32 {
        match sex {
            Sex::Male => 10.0 * weight_kg + 6.25 * height_cm - 5.0 * age as f32 + 5.0,
            Sex::Female => 10.0 * weight_kg + 6.25 * height_cm - 5.0 * age as f32 - 161.0,
        }
    }

    pub fn calculate_tdee(bmr: f32, activity_level: ActivityLevel) -> f32 {
        bmr * activity_level.multiplier()
    }

    pub fn calculate_macros(tdee: f32, goal: Goal) -> Macros {
        let (protein_pct, fat_pct, carb_pct) = match goal {
            Goal::LoseWeight => (0.3, 0.25, 0.45),
            Goal::Maintain => (0.25, 0.25, 0.5),
            Goal::GainMuscle => (0.3, 0.2, 0.5),
        };

        let protein = tdee * protein_pct / 4.0;
        let fat = tdee * fat_pct / 9.0;
        let carbs = tdee * carb_pct / 4.0;

        Macros {
            protein,
            fat,
            carbs,
            calories: tdee,
        }
    }
}
