use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Type)]
#[sqlx(type_name = "sex", rename_all = "snake_case")]
pub enum SexRow {
    Male,
    Female,
}

#[derive(Debug, Clone, Copy, Type)]
#[sqlx(type_name = "activity_level", rename_all = "snake_case")]
pub enum ActivityLevelRow {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtraActive,
}

#[derive(Debug, Clone, Copy, Type)]
#[sqlx(type_name = "goal", rename_all = "snake_case")]
pub enum GoalRow {
    LoseWeight,
    Maintain,
    GainMuscle,
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub sex: SexRow,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: ActivityLevelRow,
    pub goal: GoalRow,
}
#[derive(Debug, FromRow)]
pub struct SignInUserRow {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}
