use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserDto {
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: String,
    pub goal: String,
}

impl UpdateUserDto {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.weight <= 0.0 || self.weight > 300.0 {
            errors.push("weight must be between 0 and 300 kg".to_string());
        }

        if self.height <= 0 || self.height > 250 {
            errors.push("height must be between 0 and 250 cm".to_string());
        }

        if self.age <= 0 || self.age > 120 {
            errors.push("age must be between 0 and 120 years".to_string());
        }

        let valid_activity = [
            "sedentary",
            "lightly_active",
            "moderately_active",
            "very_active",
            "extra_active",
        ];
        if !valid_activity.contains(&self.activity_level.to_lowercase().as_str()) {
            errors.push("activity_level must be one of: sedentary, lightly_active, moderately_active, very_active, extra_active".to_string());
        }

        let valid_goals = ["lose_weight", "maintain", "gain_muscle"];
        if !valid_goals.contains(&self.goal.to_lowercase().as_str()) {
            errors.push("goal must be one of: lose_weight, maintain, gain_muscle".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
