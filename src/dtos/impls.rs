use super::user_nutrition_goals_dto::UserNutritionsGoalsDto;
use crate::domain::nutrition::goals::NutritionGoals;

impl From<UserNutritionsGoalsDto> for NutritionGoals {
    fn from(dto: UserNutritionsGoalsDto) -> Self {
        NutritionGoals::new(
            dto.user_id,
            dto.protein_goal,
            dto.fats_goal,
            dto.carbs_goal,
            dto.tdee,
            dto.bmr,
        )
    }
}
