use sqlx::PgPool;

use crate::application::auth::login::LoginUseCase;
use crate::application::auth::refresh_token::RefreshTokenUseCase;
use crate::application::auth::verify_token::VerifyTokenUseCase;
use crate::application::user::change_password::ChangePasswordUseCase;
use crate::application::user::register_user::RegisterUserUseCase;
use crate::application::user::update_goals::UpdateGoalsUseCase;
use crate::application::user::update_user::UpdateUserUseCase;
use crate::auth::jwt::Jwt;
use crate::infrastructure::persistence::repositories::SqlxNutritionRepository;
use crate::infrastructure::persistence::repositories::SqlxUserRepository;

#[derive(Clone)]
pub struct Services {
    pub register_user_use_case: RegisterUserUseCase<SqlxUserRepository, SqlxNutritionRepository>,
    pub login_use_case: LoginUseCase<SqlxUserRepository>,
    pub verify_token_use_case: VerifyTokenUseCase<SqlxUserRepository>,
    pub refresh_token_use_case: RefreshTokenUseCase<SqlxUserRepository>,
    pub change_password_use_case: ChangePasswordUseCase<SqlxUserRepository>,
    pub update_goals_use_case: UpdateGoalsUseCase<SqlxNutritionRepository>,
    pub update_user_use_case: UpdateUserUseCase<SqlxUserRepository, SqlxNutritionRepository>,
    pub user_repository: SqlxUserRepository,
    pub goals_repository: SqlxNutritionRepository,
}

impl Services {
    pub fn new(pool: PgPool, secret_key: String) -> Self {
        let jwt = Jwt::new(secret_key);

        let user_repository = SqlxUserRepository::new(pool.clone());
        let goals_repository = SqlxNutritionRepository::new(pool.clone());

        let user_repo_clone = user_repository.clone();
        let goals_repo_clone = goals_repository.clone();
        let jwt_clone = jwt.clone();

        Self {
            register_user_use_case: RegisterUserUseCase::new(
                user_repository.clone(),
                goals_repository.clone(),
                jwt.clone(),
            ),
            login_use_case: LoginUseCase::new(user_repo_clone.clone(), jwt_clone.clone()),
            verify_token_use_case: VerifyTokenUseCase::new(
                user_repo_clone.clone(),
                jwt_clone.clone(),
            ),
            refresh_token_use_case: RefreshTokenUseCase::new(user_repo_clone, jwt),
            change_password_use_case: ChangePasswordUseCase::new(user_repository.clone()),
            update_goals_use_case: UpdateGoalsUseCase::new(goals_repo_clone.clone()),
            update_user_use_case: UpdateUserUseCase::new(
                user_repository.clone(),
                goals_repository.clone(),
            ),
            user_repository,
            goals_repository,
        }
    }
}
