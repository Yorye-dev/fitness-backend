use crate::application::{
    auth::{
        login::LoginUseCase, refresh_token::RefreshTokenUseCase, verify_token::VerifyTokenUseCase,
    },
    hydration::Hydration,
    nutrition::{
        create_meal::CreateMealUseCase,
        delete_consumption::DeleteConsumptionUseCase,
        delete_meal::DeleteMealUseCase,
        get_daily_nutrition::GetDailyNutritionUseCase,
        get_goals::GetGoalsUseCase,
        get_meal::{GetMealUseCase, ListMealsUseCase},
        log_consumption::LogConsumptionUseCase,
        update_consumption::UpdateConsumptionUseCase,
        update_goals::UpdateGoalsUseCase,
        update_meal::UpdateMealUseCase,
    },
    security::{password_service::PasswordService, token_service::TokenService},
    training::{
        routines::{ArchiveRoutineUseCase, ListRoutinesUseCase, SaveRoutineUseCase},
        schedule::{GetDailyWorkoutUseCase, GetWeeklyScheduleUseCase, SaveWeeklyScheduleUseCase},
        sessions::WorkoutSessions,
    },
    user::{
        change_password::ChangePasswordUseCase, get_current_user::GetCurrentUserUseCase,
        register_user::RegisterUserUseCase, update_user::UpdateUserUseCase,
    },
};
use crate::domain::{
    hydration::repository::HydrationRepository,
    nutrition::repository::{ConsumptionRepository, MealRepository, NutritionRepository},
    training::repository::TrainingRepository,
    training::session_repository::WorkoutSessionRepository,
    user::repository::UserRepository,
};
use crate::infrastructure::{
    auth::jwt::JwtTokenService,
    persistence::repositories::{
        SqlxHydrationRepository, SqlxNutritionRepository, SqlxTrainingRepository,
        SqlxUserRepository, SqlxWorkoutSessionRepository,
    },
    security::password::Argon2PasswordService,
};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppDependencies {
    pub hydration: Arc<dyn HydrationRepository>,
    pub workout_sessions: Arc<dyn WorkoutSessionRepository>,
    pub users: Arc<dyn UserRepository>,
    pub nutrition: Arc<dyn NutritionRepository>,
    pub meals: Arc<dyn MealRepository>,
    pub consumptions: Arc<dyn ConsumptionRepository>,
    pub tokens: Arc<dyn TokenService>,
    pub passwords: Arc<dyn PasswordService>,
    pub training: Arc<dyn TrainingRepository>,
}

#[derive(Clone)]
pub struct AppState {
    pub(crate) hydration: Hydration,
    pub(crate) workout_sessions: WorkoutSessions,
    pub(crate) list_routines_use_case: ListRoutinesUseCase,
    pub(crate) save_routine_use_case: SaveRoutineUseCase,
    pub(crate) archive_routine_use_case: ArchiveRoutineUseCase,
    pub(crate) get_weekly_schedule_use_case: GetWeeklyScheduleUseCase,
    pub(crate) save_weekly_schedule_use_case: SaveWeeklyScheduleUseCase,
    pub(crate) get_daily_workout_use_case: GetDailyWorkoutUseCase,
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
    pub(crate) log_consumption_use_case: LogConsumptionUseCase,
    pub(crate) update_consumption_use_case: UpdateConsumptionUseCase,
    pub(crate) delete_consumption_use_case: DeleteConsumptionUseCase,
}
impl AppState {
    pub fn new(pool: PgPool, secret_key: String) -> Self {
        let nutrition = Arc::new(SqlxNutritionRepository::new(pool.clone()));
        Self::from_dependencies(AppDependencies {
            hydration: Arc::new(SqlxHydrationRepository::new(pool.clone())),
            workout_sessions: Arc::new(SqlxWorkoutSessionRepository::new(pool.clone())),
            training: Arc::new(SqlxTrainingRepository::new(pool.clone())),
            users: Arc::new(SqlxUserRepository::new(pool)),
            nutrition: nutrition.clone(),
            meals: nutrition.clone(),
            consumptions: nutrition,
            tokens: Arc::new(JwtTokenService::new(&secret_key)),
            passwords: Arc::new(Argon2PasswordService),
        })
    }
    pub fn from_dependencies(deps: AppDependencies) -> Self {
        let goals = GetGoalsUseCase::new(deps.nutrition.clone(), deps.users.clone());
        Self {
            hydration: Hydration::new(deps.hydration),
            workout_sessions: WorkoutSessions::new(deps.workout_sessions),
            list_routines_use_case: ListRoutinesUseCase::new(deps.training.clone()),
            save_routine_use_case: SaveRoutineUseCase::new(deps.training.clone()),
            archive_routine_use_case: ArchiveRoutineUseCase::new(deps.training.clone()),
            get_weekly_schedule_use_case: GetWeeklyScheduleUseCase::new(deps.training.clone()),
            save_weekly_schedule_use_case: SaveWeeklyScheduleUseCase::new(deps.training.clone()),
            get_daily_workout_use_case: GetDailyWorkoutUseCase::new(deps.training),
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
            log_consumption_use_case: LogConsumptionUseCase::new(deps.consumptions.clone()),
            update_consumption_use_case: UpdateConsumptionUseCase::new(deps.consumptions.clone()),
            delete_consumption_use_case: DeleteConsumptionUseCase::new(deps.consumptions),
        }
    }
}
