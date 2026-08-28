use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct RegisterUserDto {
    pub username: String,
    pub plain_password: String,
    pub sex: String,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: String,
    pub goal: String,
}

impl RegisterUserDto {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.username.is_empty() {
            errors.push("username cannot be empty".to_string());
        }
        if self.username.len() < 3 || self.username.len() > 50 {
            errors.push("username must be between 3 and 50 characters".to_string());
        }
        if !self
            .username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            errors.push(
                "username can only contain alphanumeric characters and underscores".to_string(),
            );
        }

        if self.plain_password.is_empty() {
            errors.push("password cannot be empty".to_string());
        }
        if self.plain_password.len() < 8 {
            errors.push("password must be at least 8 characters".to_string());
        }

        let valid_sex = ["male", "female"];
        if !valid_sex.contains(&self.sex.to_lowercase().as_str()) {
            errors.push("sex must be 'male' or 'female'".to_string());
        }

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
