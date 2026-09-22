use serde::Deserialize;

use crate::application::user::register_user::RegisterUserInput;
use crate::domain::enums::{activity_level::ActivityLevel, goals::Goal, sex::Sex};

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub plain_password: String,
    pub sex: String,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: String,
    pub goal: String,
}

impl TryFrom<RegisterUserRequest> for RegisterUserInput {
    type Error = Vec<String>;

    fn try_from(request: RegisterUserRequest) -> Result<Self, Self::Error> {
        let mut errors = Vec::new();

        if request.username.is_empty() {
            errors.push("username cannot be empty".to_string());
        }

        if request.username.len() < 3 || request.username.len() > 50 {
            errors.push("username must be between 3 and 50 characters".to_string());
        }

        if !request
            .username
            .chars()
            .all(|character| character.is_alphanumeric() || character == '_')
        {
            errors.push(
                "username can only contain alphanumeric characters and underscores".to_string(),
            );
        }

        if request.plain_password.is_empty() {
            errors.push("password cannot be empty".to_string());
        }

        if request.plain_password.len() < 8 {
            errors.push("password must be at least 8 characters".to_string());
        }

        if request.weight <= 0.0 || request.weight > 300.0 {
            errors.push("weight must be between 0 and 300 kg".to_string());
        }

        if request.height <= 0 || request.height > 250 {
            errors.push("height must be between 0 and 250 cm".to_string());
        }

        if request.age <= 0 || request.age > 120 {
            errors.push("age must be between 0 and 120 years".to_string());
        }

        let sex = match request.sex.to_ascii_lowercase().as_str() {
            "male" => Some(Sex::Male),
            "female" => Some(Sex::Female),

            _ => {
                errors.push("sex must be 'male' or 'female'".to_string());

                None
            }
        };

        let activity_level = match request.activity_level.to_ascii_lowercase().as_str() {
            "sedentary" => Some(ActivityLevel::Sedentary),

            "lightly_active" => Some(ActivityLevel::LightlyActive),

            "moderately_active" => Some(ActivityLevel::ModeratelyActive),

            "very_active" => Some(ActivityLevel::VeryActive),

            "extra_active" => Some(ActivityLevel::ExtraActive),

            _ => {
                errors.push(
                        "activity_level must be one of: sedentary, lightly_active, moderately_active, very_active, extra_active"
                            .to_string(),
                    );

                None
            }
        };

        let goal = match request.goal.to_ascii_lowercase().as_str() {
            "lose_weight" => Some(Goal::LoseWeight),
            "maintain" => Some(Goal::Maintain),
            "gain_muscle" => Some(Goal::GainMuscle),

            _ => {
                errors.push("goal must be one of: lose_weight, maintain, gain_muscle".to_string());

                None
            }
        };

        if !errors.is_empty() {
            return Err(errors);
        }

        let (Some(sex), Some(activity_level), Some(goal)) = (sex, activity_level, goal) else {
            return Err(vec!["invalid register request".to_string()]);
        };

        Ok(Self {
            username: request.username,
            plain_password: request.plain_password,
            sex,
            weight: request.weight,
            height: request.height,
            age: request.age,
            activity_level,
            goal,
        })
    }
}
