use crate::domain::user::{activity_level::ActivityLevel, entity::User, goal::Goal, sex::Sex};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub sex: &'static str,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: &'static str,
    pub goal: &'static str,
}
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        let profile = user.profile();
        Self {
            id: user.id(),
            username: user.username().to_owned(),
            weight: profile.weight(),
            height: profile.height(),
            age: profile.age(),
            sex: match user.sex() {
                Sex::Male => "Male",
                Sex::Female => "Female",
            },
            activity_level: match profile.activity_level() {
                ActivityLevel::Sedentary => "Sedentary",
                ActivityLevel::LightlyActive => "LightlyActive",
                ActivityLevel::ModeratelyActive => "ModeratelyActive",
                ActivityLevel::VeryActive => "VeryActive",
                ActivityLevel::ExtraActive => "ExtraActive",
            },
            goal: match profile.goal() {
                Goal::LoseWeight => "LoseWeight",
                Goal::Maintain => "Maintain",
                Goal::GainMuscle => "GainMuscle",
            },
        }
    }
}
