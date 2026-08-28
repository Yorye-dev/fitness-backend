use serde::Deserialize;

#[derive(Deserialize)]
pub struct LogConsumptionDto {
    pub meal_id: String,
    pub quantity_grams: f32,
    pub date: Option<String>,
}

impl LogConsumptionDto {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.meal_id.is_empty() {
            errors.push("meal_id cannot be empty".to_string());
        }

        if self.quantity_grams <= 0.0 {
            errors.push("quantity_grams must be greater than 0".to_string());
        }
        if self.quantity_grams > 5000.0 {
            errors.push("quantity_grams cannot exceed 5000g".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
