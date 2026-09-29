use crate::application::{
    auth::login::AuthTokens,
    errors::ApplicationError,
    security::{password_service::PasswordService, token_service::TokenService},
};
use crate::domain::nutrition::calculator::NutritionCalculator;
use crate::domain::user::{
    activity_level::ActivityLevel,
    factory::{NewUserData, UserFactory},
    goal::Goal,
    profile::UserProfile,
    repository::UserRepository,
    sex::Sex,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct RegisterUserInput {
    pub username: String,
    pub plain_password: String,
    pub sex: Sex,
    pub weight: f32,
    pub height: i32,
    pub age: i32,
    pub activity_level: ActivityLevel,
    pub goal: Goal,
}
#[derive(Clone)]
pub struct RegisterUserUseCase {
    users: Arc<dyn UserRepository>,
    tokens: Arc<dyn TokenService>,
    passwords: Arc<dyn PasswordService>,
}
impl RegisterUserUseCase {
    pub fn new(
        users: Arc<dyn UserRepository>,
        tokens: Arc<dyn TokenService>,
        passwords: Arc<dyn PasswordService>,
    ) -> Self {
        Self {
            users,
            tokens,
            passwords,
        }
    }
    pub async fn execute(&self, input: RegisterUserInput) -> Result<AuthTokens, ApplicationError> {
        let profile = UserProfile::new(
            input.weight,
            input.height,
            input.age,
            input.activity_level,
            input.goal,
        )?;
        super::change_password::validate_new_password(&input.plain_password)?;
        let user = UserFactory::create_user(NewUserData {
            username: input.username,
            password_hash: self.passwords.hash(&input.plain_password).await?,
            sex: input.sex,
            profile,
        });
        let goals = NutritionCalculator::goals_for(&user);
        let tokens = AuthTokens {
            access_token: self.tokens.create_access_token(&user.id())?,
            refresh_token: self.tokens.create_refresh_token(&user.id())?,
        };
        self.users.save_user(&user, &goals).await?;
        Ok(tokens)
    }
}
