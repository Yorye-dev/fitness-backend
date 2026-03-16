use crate::auth::claims::Claims;
use crate::dtos::register_user_dto::RegisterUserDto;
use crate::domain::user::repository::UserRepository as UserRepositoryTrait;
use crate::domain::nutrition::repository::NutritionRepository as NutritionRepositoryTrait;
use crate::errors::AuthError;
use crate::domain::user::factory::UserFactory;
use crate::domain::nutrition::goals::NutritionGoals;
use crate::domain::nutrition::calculator::NutritionCalculator;
use crate::auth;
use crate::dtos::sign_data_dto::SignInData;
use crate::auth::jwt::Jwt;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<dyn UserRepositoryTrait>,
    goals_repo: Arc<dyn NutritionRepositoryTrait>,
    jwt: Jwt,
}

impl AuthService {

    pub fn new (
        user_repo: Arc<dyn UserRepositoryTrait>, 
        goals_repo: Arc<dyn NutritionRepositoryTrait>, 
        jwt: Jwt
    ) -> Self {
        
        Self { user_repo, goals_repo, jwt }
    }

    pub async fn sing_in (&self, dto :SignInData) -> Result<String, AuthError> {
        
        let user = self.user_repo
            .get_sign_in_user_by_username(&dto.username)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        if !auth::utils::verify_password(&dto.password, &user.password_hash){
            return Err(AuthError::InvalidCredentials);
        }

        let token_data = self.jwt.generate_token(&user.id, 60)
            .map_err(|_| AuthError::GenerateTokenError);

        Ok(token_data?)
    }

    pub async fn register(&self, dto :RegisterUserDto) -> Result<String, AuthError> {

        let user = UserFactory::create_user_from_dto(dto).unwrap();

        let user_goals = Self::generate_user_goals(&user);

        self.user_repo.save_user(&user).await?;
        self.goals_repo.save_user_goals(&user_goals).await?;

        let token_data = self.jwt.generate_token(&user.id, 60)
            .map_err(|_| AuthError::GenerateTokenError);

        Ok(token_data?)        
    }

    pub async fn get_claims_if_valid(&self, token :String) -> Result<Claims, AuthError> {

        let claims  = self.jwt.decode_token(&token).map_err(|_| AuthError::GenerateTokenError)?;

        let user_id = claims.subject.clone();

        let exists = self.user_repo.exists(&user_id)
            .await
            .map_err(|_| AuthError::UserNotFound)?;

        if !exists {
            return Err(AuthError::UserNotFound)
        }

        Ok(claims)
    }

    fn generate_user_goals(user: &crate::domain::user::user::User) -> NutritionGoals {
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
