use uuid::Uuid;

use crate::models::user::User;
use crate::dtos::register_user_dto::RegisterUserDto;
use crate::utils::password_utils::calculate_hash;

pub struct UserFactory;

impl UserFactory {
    pub fn create_user_from_dto(dto: RegisterUserDto) -> User{
        
        // TODO: Sustituri por un llamada a los helpers.

        let password_hash = calculate_hash(&dto.plain_password).unwrap();

        User {
            id: Uuid::new_v4().to_string(),
            username: dto.username,
            password_hash,
            age: dto.age,
            sex: dto.sex,
            height: dto.height,
            weight: dto.weight,
            activity_level: dto.activity_level,
        }
    }
}
