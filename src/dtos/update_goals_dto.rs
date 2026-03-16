use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGoalsDto {
    pub protein_goal: f32,
    pub carbs_goal: f32,
    pub fats_goal: f32,
}

impl UpdateGoalsDto {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.protein_goal < 0.0 {
            errors.push("protein_goal cannot be negative".to_string());
        }
        if self.carbs_goal < 0.0 {
            errors.push("carbs_goal cannot be negative".to_string());
        }
        if self.fats_goal < 0.0 {
            errors.push("fats_goal cannot be negative".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
