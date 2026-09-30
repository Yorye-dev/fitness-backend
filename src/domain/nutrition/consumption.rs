use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DailyConsumption {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: chrono::NaiveDate,
    pub meal_id: Uuid,
    pub quantity_grams: Option<f64>,
    pub portion_count: Option<f64>,
    pub portion_grams: Option<f64>,
    pub calories_consumed: f32,
    pub protein_consumed: f32,
    pub carbs_consumed: f32,
    pub fat_consumed: f32,
}

#[derive(Debug, Clone)]
pub struct ConsumptionQuantity {
    pub quantity_grams: Option<f64>,
    pub portion_count: Option<f64>,
    pub portion_grams: Option<f64>,
}
