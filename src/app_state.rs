use sqlx::PgPool;

use crate::application::auth::login::LoginUseCase;
use crate::application::auth::refresh_token::RefreshTokenUseCase;
use crate::application::auth::verify_token::VerifyTokenUseCase;
use crate::application::user::change_password::ChangePasswordUseCase;
use crate::application::user::register_user::RegisterUserUseCase;
use crate::application::user::update_goals::UpdateGoalsUseCase;
use crate::application::user::update_user::UpdateUserUseCase;
use crate::application::user::get_current_user::GetCurrentUserUseCase;

use crate::auth::jwt::Jwt;

use crate::infrastructure::persistence::repositories::{
    SqlxNutritionRepository,
    SqlxUserRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub register_user_use_case:
        RegisterUserUseCase<SqlxUserRepository, SqlxNutritionRepository>,

    pub login_use_case:
        LoginUseCase<SqlxUserRepository>,

    pub verify_token_use_case:
        VerifyTokenUseCase<SqlxUserRepository>,

    pub refresh_token_use_case:
        RefreshTokenUseCase<SqlxUserRepository>,

    pub change_password_use_case:
        ChangePasswordUseCase<SqlxUserRepository>,

    pub update_goals_use_case:
        UpdateGoalsUseCase<SqlxNutritionRepository>,

    pub update_user_use_case:
        UpdateUserUseCase<SqlxUserRepository, SqlxNutritionRepository>,

    pub get_current_user_use_case:
        GetCurrentUserUseCase<SqlxUserRepository>,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        secret_key: String,
    ) -> Self {
        let jwt = Jwt::new(secret_key);

        let user_repository =
            SqlxUserRepository::new(pool.clone());

        let nutrition_repository =
            SqlxNutritionRepository::new(pool);

        Self {
            register_user_use_case:
                RegisterUserUseCase::new(
                    user_repository.clone(),
                    nutrition_repository.clone(),
                    jwt.clone(),
                ),

            login_use_case:
                LoginUseCase::new(
                    user_repository.clone(),
                    jwt.clone(),
                ),

            verify_token_use_case:
                VerifyTokenUseCase::new(
                    user_repository.clone(),
                    jwt.clone(),
                ),

            refresh_token_use_case:
                RefreshTokenUseCase::new(
                    user_repository.clone(),
                    jwt,
                ),
            get_current_user_use_case:
                GetCurrentUserUseCase::new(
                    user_repository.clone(),
                ),
            change_password_use_case:
                ChangePasswordUseCase::new(
                    user_repository.clone(),
                ),
            update_goals_use_case:
                UpdateGoalsUseCase::new(
                    nutrition_repository.clone(),
                ),

            update_user_use_case:
                UpdateUserUseCase::new(
                    user_repository,
                    nutrition_repository,
                ),
        }
    }
}
