pub mod repository;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct WaterEntry {
    pub id: Uuid,
    pub amount_ml: i32,
}
pub struct DailyWater {
    pub date: NaiveDate,
    pub goal_ml: i32,
    pub total_ml: i64,
    pub entries: Vec<WaterEntry>,
}
