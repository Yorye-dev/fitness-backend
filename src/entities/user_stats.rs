use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct UserStats {

    pub id: Uuid,
    pub user_id: Uuid,

    pub stat_date: NaiveDate,

    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fats: f32,
    pub total_fiber: f32,
    pub total_sugar: f32,
    pub total_sodium: f32,

    pub goal_calories: Option<f32>,
    pub goal_protein: Option<f32>,
    pub goal_carbs: Option<f32>,
    pub goal_fats: Option<f32>,
    pub goal_fiber: Option<f32>,
    pub goal_sugar: Option<f32>,
    pub goal_sodium: Option<f32>,

    pub progress_calories: Option<f32>,
    pub progress_protein: Option<f32>,
    pub progress_carbs: Option<f32>,
    pub progress_fats: Option<f32>,
    pub progress_fiber: Option<f32>,
    pub progress_sugar: Option<f32>,
    pub progress_sodium: Option<f32>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

