use crate::application::nutrition::update_goals::UpdateGoalsInput;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateGoalsRequest {
    pub protein_goal: f32,
    pub fats_goal: f32,
    pub carbs_goal: f32,
    pub tdee: f32,
}
impl From<UpdateGoalsRequest> for UpdateGoalsInput {
    fn from(request: UpdateGoalsRequest) -> Self {
        Self {
            protein_goal: request.protein_goal,
            fats_goal: request.fats_goal,
            carbs_goal: request.carbs_goal,
            tdee: request.tdee,
        }
    }
}
