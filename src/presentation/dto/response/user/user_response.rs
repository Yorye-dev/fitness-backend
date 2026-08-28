use serde::Serialize;
use uuid::Uuid;

use crate::domain::enums::{
    activity_level::ActivityLevel,
    goals::Goal,
    sex::Sex,
};
use crate::domain::user::user::User;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub sex: Sex,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: ActivityLevel,
    pub goal: Goal,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            sex: user.sex,
            weight: user.weight,
            height: user.height,
            age: user.age,
            activity_level: user.activity_level,
            goal: user.goal,
        }
    }
}
