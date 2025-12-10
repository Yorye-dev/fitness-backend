
use uuid::Uuid;

use crate::enums::{sex::Sex, activity_level::ActivityLevel, goals::Goal};
use crate::entities::user::User;
use crate::dtos::register_user_dto::RegisterUserDto;
use crate::utils::password_utils::calculate_hash;

pub struct UserFactory;

impl UserFactory {
    pub fn create_user_from_dto(dto: RegisterUserDto) -> Result<User, String> {
        let password_hash = calculate_hash(&dto.plain_password).unwrap();

        // Convertir strings a enums
        let sex_enum = match dto.sex.to_lowercase().as_str() {
            "male" => Sex::Male,
            "female" => Sex::Female,
            _ => return Err("Invalid sex".to_string()),
        };

        let activity_enum = match dto.activity_level.to_lowercase().as_str() {
            "sedentary" => ActivityLevel::Sedentary,
            "lightly_active" => ActivityLevel::LightlyActive,
            "moderately_active" => ActivityLevel::ModeratelyActive,
            "very_active" => ActivityLevel::VeryActive,
            "extra_active" => ActivityLevel::ExtraActive,
            _ => return Err("Invalid activity level".to_string()),
        };

        let goal_enum = match dto.goal.to_lowercase().as_str() {
            "lose_weight" => Goal::LoseWeight,
            "maintain" => Goal::Maintain,
            "gain_muscle" => Goal::GainMuscle,
            _ => return Err("Invalid goal".to_string()),
        };

        Ok(User {
            id: Uuid::new_v4(),
            username: dto.username,
            password_hash,
            age: dto.age,
            sex: sex_enum,
            height: dto.height,
            weight: dto.weight,
            activity_level: activity_enum,
            goal: goal_enum,
        })
    }
}
