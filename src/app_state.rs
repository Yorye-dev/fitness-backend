use crate::application::{
    auth::{
        login::LoginUseCase, refresh_token::RefreshTokenUseCase, verify_token::VerifyTokenUseCase,
    },
    nutrition::{
        create_meal::CreateMealUseCase,
        delete_meal::DeleteMealUseCase,
        get_daily_nutrition::GetDailyNutritionUseCase,
        get_goals::GetGoalsUseCase,
        get_meal::{GetMealUseCase, ListMealsUseCase},
        update_goals::UpdateGoalsUseCase,
        update_meal::UpdateMealUseCase,
    },
    security::{password_service::PasswordService, token_service::TokenService},
    user::{
        change_password::ChangePasswordUseCase, get_current_user::GetCurrentUserUseCase,
        register_user::RegisterUserUseCase, update_user::UpdateUserUseCase,
    },
};
use crate::domain::{
    nutrition::repository::{MealRepository, NutritionRepository},
    user::repository::UserRepository,
};
use crate::infrastructure::{
    auth::jwt::JwtTokenService,
    persistence::repositories::{SqlxNutritionRepository, SqlxUserRepository},
    security::password::Argon2PasswordService,
};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppDependencies {
    pub users: Arc<dyn UserRepository>,
    pub nutrition: Arc<dyn NutritionRepository>,
    pub meals: Arc<dyn MealRepository>,
    pub tokens: Arc<dyn TokenService>,
    pub passwords: Arc<dyn PasswordService>,
}

#[derive(Clone)]
pub struct AppState {
    pub(crate) register_user_use_case: RegisterUserUseCase,
    pub(crate) login_use_case: LoginUseCase,
    pub(crate) verify_token_use_case: VerifyTokenUseCase,
    pub(crate) refresh_token_use_case: RefreshTokenUseCase,
    pub(crate) change_password_use_case: ChangePasswordUseCase,
    pub(crate) update_user_use_case: UpdateUserUseCase,
    pub(crate) get_current_user_use_case: GetCurrentUserUseCase,
    pub(crate) get_daily_nutrition_use_case: GetDailyNutritionUseCase,
    pub(crate) get_goals_use_case: GetGoalsUseCase,
    pub(crate) update_goals_use_case: UpdateGoalsUseCase,
    pub(crate) create_meal_use_case: CreateMealUseCase,
    pub(crate) get_meal_use_case: GetMealUseCase,
    pub(crate) list_meals_use_case: ListMealsUseCase,
    pub(crate) update_meal_use_case: UpdateMealUseCase,
    pub(crate) delete_meal_use_case: DeleteMealUseCase,
}
impl AppState {
    pub fn new(pool: PgPool, secret_key: String) -> Self {
        let nutrition = Arc::new(SqlxNutritionRepository::new(pool.clone()));
        Self::from_dependencies(AppDependencies {
            users: Arc::new(SqlxUserRepository::new(pool)),
            nutrition: nutrition.clone(),
            meals: nutrition,
            tokens: Arc::new(JwtTokenService::new(&secret_key)),
            passwords: Arc::new(Argon2PasswordService),
        })
    }
    pub fn from_dependencies(deps: AppDependencies) -> Self {
        let goals = GetGoalsUseCase::new(deps.nutrition.clone(), deps.users.clone());
        Self {
            register_user_use_case: RegisterUserUseCase::new(
                deps.users.clone(),
                deps.tokens.clone(),
                deps.passwords.clone(),
            ),
            login_use_case: LoginUseCase::new(
                deps.users.clone(),
                deps.tokens.clone(),
                deps.passwords.clone(),
            ),
            verify_token_use_case: VerifyTokenUseCase::new(deps.users.clone(), deps.tokens.clone()),
            refresh_token_use_case: RefreshTokenUseCase::new(deps.users.clone(), deps.tokens),
            change_password_use_case: ChangePasswordUseCase::new(
                deps.users.clone(),
                deps.passwords,
            ),
            update_user_use_case: UpdateUserUseCase::new(deps.users.clone()),
            get_current_user_use_case: GetCurrentUserUseCase::new(deps.users.clone()),
            get_daily_nutrition_use_case: GetDailyNutritionUseCase::new(
                deps.nutrition.clone(),
                goals.clone(),
            ),
            get_goals_use_case: goals,
            update_goals_use_case: UpdateGoalsUseCase::new(deps.nutrition, deps.users),
            create_meal_use_case: CreateMealUseCase::new(deps.meals.clone()),
            get_meal_use_case: GetMealUseCase::new(deps.meals.clone()),
            list_meals_use_case: ListMealsUseCase::new(deps.meals.clone()),
            update_meal_use_case: UpdateMealUseCase::new(deps.meals.clone()),
            delete_meal_use_case: DeleteMealUseCase::new(deps.meals),
        }
    }
}
