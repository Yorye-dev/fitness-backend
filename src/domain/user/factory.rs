use uuid::Uuid;

use crate::domain::enums::{activity_level::ActivityLevel, goals::Goal, sex::Sex};
use crate::domain::user::user::User;

pub struct NewUserData {
    pub username: String,
    pub password_hash: String,
    pub sex: Sex,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: ActivityLevel,
    pub goal: Goal,
}

pub struct UserFactory;

impl UserFactory {
    pub fn create_user(data: NewUserData) -> User {
        User {
            id: Uuid::new_v4(),
            username: data.username,
            password_hash: data.password_hash,
            age: data.age,
            sex: data.sex,
            height: data.height,
            weight: data.weight,
            activity_level: data.activity_level,
            goal: data.goal,
        }
    }
}
