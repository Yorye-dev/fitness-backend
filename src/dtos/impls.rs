use super::user_nutrition_goals_dto::UserNutritionsGoalsDto;
use crate::entities::user_nutrition_goals::UserNutritionsGoals;

impl From<UserNutritionsGoalsDto> for UserNutritionsGoals {
    fn from(dto: UserNutritionsGoalsDto) -> Self {
        UserNutritionsGoals::new(
            dto.user_id,
            dto.protein_goal,
            dto.fats_goal,
            dto.carbs_goal,
            dto.tdee,
            dto.bmr,
        )
    }
}
