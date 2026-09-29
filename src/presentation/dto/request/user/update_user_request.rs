use crate::{
    application::user::update_user::UpdateUserInput,
    domain::user::{activity_level::ActivityLevel, goal::Goal},
    presentation::errors::api_error::ApiError,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserRequest {
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: String,
    pub goal: String,
}
impl TryFrom<UpdateUserRequest> for UpdateUserInput {
    type Error = ApiError;
    fn try_from(request: UpdateUserRequest) -> Result<Self, Self::Error> {
        let activity_level = match request.activity_level.to_ascii_lowercase().as_str() {
            "sedentary" => ActivityLevel::Sedentary,
            "lightly_active" => ActivityLevel::LightlyActive,
            "moderately_active" => ActivityLevel::ModeratelyActive,
            "very_active" => ActivityLevel::VeryActive,
            "extra_active" => ActivityLevel::ExtraActive,
            _ => return Err(ApiError::Validation("invalid activity_level".into())),
        };
        let goal = match request.goal.to_ascii_lowercase().as_str() {
            "lose_weight" => Goal::LoseWeight,
            "maintain" => Goal::Maintain,
            "gain_muscle" => Goal::GainMuscle,
            _ => return Err(ApiError::Validation("invalid goal".into())),
        };
        Ok(Self {
            weight: request.weight,
            height: request.height,
            age: request.age,
            activity_level,
            goal,
        })
    }
}
