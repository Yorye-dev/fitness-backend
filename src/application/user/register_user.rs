use async_trait::async_trait;
use crate::domain::errors::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user::User;
use crate::domain::nutrition::repository::NutritionRepository;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::user::factory::UserFactory;
use crate::domain::nutrition::calculator::NutritionCalculator;
use crate::dtos::register_user_dto::RegisterUserDto;
use crate::auth::jwt::Jwt;

pub struct RegisterUserUseCase<R: UserRepository, N: NutritionRepository> {
    user_repo: R,
    nutrition_repo: N,
    jwt: Jwt,
}

impl<R: UserRepository, N: NutritionRepository> RegisterUserUseCase<R, N> {
    pub fn new(user_repo: R, nutrition_repo: N, jwt: Jwt) -> Self {
        Self { user_repo, nutrition_repo, jwt }
    }

    pub async fn execute(&self, dto: RegisterUserDto) -> Result<String, DomainError> {
        let user = UserFactory::create_user_from_dto(dto)
            .map_err(|e| DomainError::ValidationError(e))?;

        let goals = Self::generate_user_goals(&user);

        self.user_repo.save_user(&user).await?;
        self.nutrition_repo.save_user_goals(&goals).await?;

        let token = self.jwt.generate_token(&user.id.to_string(), 60)
            .map_err(|_| DomainError::GenerateTokenError)?;

        Ok(token)
    }

    fn generate_user_goals(user: &User) -> NutritionGoals {
        let bmr = NutritionCalculator::calculate_bmr(
            user.weight, 
            user.height as f32, 
            user.age as u32, 
            user.sex.clone()
        );
        let tdee = NutritionCalculator::calculate_tdee(bmr, user.activity_level.clone());
        let macros = NutritionCalculator::calculate_macros(tdee, user.goal.clone());

        NutritionGoals::new(
            user.id,
            macros.protein,
            macros.fat,
            macros.carbs,
            tdee,
            bmr
        )
    }
}
