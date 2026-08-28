use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMealDto {
    pub name: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fat_per_100g: f32,
}

impl CreateMealDto {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push("name cannot be empty".to_string());
        }
        if self.name.len() > 100 {
            errors.push("name must be at most 100 characters".to_string());
        }

        if self.calories_per_100g < 0.0 {
            errors.push("calories_per_100g cannot be negative".to_string());
        }
        if self.protein_per_100g < 0.0 {
            errors.push("protein_per_100g cannot be negative".to_string());
        }
        if self.carbs_per_100g < 0.0 {
            errors.push("carbs_per_100g cannot be negative".to_string());
        }
        if self.fat_per_100g < 0.0 {
            errors.push("fat_per_100g cannot be negative".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
