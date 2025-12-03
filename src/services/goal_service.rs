use crate::{enums::{activity_level::ActivityLevel, goals::{Goal}, sex::Sex}};
use crate::models::{user_nutrition_goals::UserNutritionsGoals ,user::User };

pub struct GoalService;
impl GoalService {

    pub fn generate_user_goals(&user: User) -> UserNutritionsGoals{

        let bmr = Self::calculate_bmr(user.weight_kg, user.height_cm, user.age, user.sex);
        let tdee = Self::calculate_tdee(bmr, user.activity_level);
        let (protein, fats, carbs) = Self::calculate_macros(tdee, user.goal);

        // Factoria de crear goals? O sacamos a dtos y dejamos solo la capa de modelos para la bdd?

    }

    fn calculate_bmr(
        weight_kg: f32, height_cm: f32, age: u32, sex: Sex
        ) -> f32 {
        match sex {
            Sex::Male => 10.0 * weight_kg + 6.25 * height_cm - 5.0 * age as f32 + 5.0,
            Sex::Female => 10.0 * weight_kg + 6.25 * height_cm - 5.0 * age as f32 - 161.0,
        }
    }

    fn calculate_tdee(bmr: f32, activity_level: ActivityLevel) -> f32 {
        bmr * activity_level.multiplier()
    }

    fn calculate_macros(tdee: f32, goal: Goal) -> (f32, f32, f32) {
        let (protein_pct, fat_pct, carb_pct) = match goal {
            Goal::LoseWeight => (0.3, 0.25, 0.45),
            Goal::Maintain => (0.25, 0.25, 0.5),
            Goal::GainMuscle => (0.3, 0.2, 0.5),
        };

        let protein = tdee * protein_pct / 4.0;
        let fats = tdee * fat_pct / 9.0;
        let carbs = tdee * carb_pct / 4.0;

        (protein, fats, carbs)
    }
}
