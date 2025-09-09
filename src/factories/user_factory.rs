use uuid::Uuid;
use chrono::Utc;
use bcrypt::{hash, DEFAULT_COST};

use crate::models::User;
use crate::dto::RegisterUserDto;

pub struct UserFactory;

impl UserFactory {
    pub fn create_user_from_dto(dto: RegisterUserDto) -> Result<User, bcrypt::BcryptError> {
        
        // TODO: Sustituri por un llamada a los helpers.

        let password_hash = hash(dto.password, DEFAULT_COST)?;

        Ok(User {
            id: Uuid::new_v4(),
            username: dto.username,
            email: dto.email,
            password_hash,
            age: dto.age,
            sex: dto.sex,
            height: dto.height,
            weight: dto.weight,
            activity_level: dto.activity_level,
            created_at: Utc::now(),
        })
    }
}
