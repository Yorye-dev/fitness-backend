use uuid::Uuid;

use crate::domain::user::entity::User;
use crate::domain::user::{profile::UserProfile, sex::Sex};

pub struct NewUserData {
    pub username: String,
    pub password_hash: String,
    pub sex: Sex,
    pub profile: UserProfile,
}

pub struct UserFactory;

impl UserFactory {
    pub fn create_user(data: NewUserData) -> User {
        User::new(
            Uuid::new_v4(),
            data.username,
            data.password_hash,
            data.sex,
            data.profile,
        )
    }
}
