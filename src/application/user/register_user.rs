use crate::application::auth::login::AuthTokens;
use crate::auth::jwt::Jwt;
use crate::domain::enums::{activity_level::ActivityLevel, goals::Goal, sex::Sex};
use crate::domain::errors::DomainError;
use crate::domain::nutrition::calculator::NutritionCalculator;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::repository::NutritionRepository;
use crate::domain::user::factory::{NewUserData, UserFactory};
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user::User;
use crate::shared::password::calculate_hash;

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
pub struct RegisterUserUseCase<R: UserRepository, N: NutritionRepository> {
    user_repo: R,
    nutrition_repo: N,
    jwt: Jwt,
}

impl<R: UserRepository, N: NutritionRepository> RegisterUserUseCase<R, N> {
    pub fn new(user_repo: R, nutrition_repo: N, jwt: Jwt) -> Self {
        Self {
            user_repo,
            nutrition_repo,
            jwt,
        }
    }

    pub async fn execute(&self, input: RegisterUserInput) -> Result<AuthTokens, DomainError> {
        let password_hash =
            calculate_hash(&input.plain_password).map_err(|_| DomainError::HashingError)?;

        let user = UserFactory::create_user(NewUserData {
            username: input.username,
            password_hash,
            sex: input.sex,
            weight: input.weight,
            height: input.height,
            age: input.age,
            activity_level: input.activity_level,
            goal: input.goal,
        });

        let goals = Self::generate_user_goals(&user);

        self.user_repo.save_user(&user).await?;
        self.nutrition_repo.save_user_goals(&goals).await?;

        let access_token = self
            .jwt
            .generate_access_token(&user.id)
            .map_err(|_| DomainError::GenerateTokenError)?;

        let refresh_token = self
            .jwt
            .generate_refresh_token(&user.id)
            .map_err(|_| DomainError::GenerateTokenError)?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
        })
    }

    fn generate_user_goals(user: &User) -> NutritionGoals {
        let bmr = NutritionCalculator::calculate_bmr(
            user.weight,
            user.height as f32,
            user.age as u32,
            user.sex.clone(),
        );

        let tdee = NutritionCalculator::calculate_tdee(bmr, user.activity_level.clone());

        let macros = NutritionCalculator::calculate_macros(tdee, user.goal.clone());

        NutritionGoals::new(user.id, macros.protein, macros.fat, macros.carbs, tdee, bmr)
    }
}
