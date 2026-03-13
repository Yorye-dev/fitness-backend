use crate::domain::enums::{activity_level::ActivityLevel, goals::Goal, sex::Sex};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub sex: Sex,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: ActivityLevel,
    pub goal: Goal,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct SignInUser {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub sex: Sex,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: ActivityLevel,
    pub goal: Goal,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser {
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
