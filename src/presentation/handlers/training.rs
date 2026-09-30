use crate::{
    app_state::AppState,
    presentation::{
        authenticated_user::AuthenticatedUser,
        dto::{
            request::training::{RoutineRequest, TrainingDateQuery, WeeklyScheduleRequest},
            response::training::{DailyWorkoutResponse, RoutineResponse, WeeklyScheduleResponse},
        },
        errors::api_error::ApiError,
        extractors::{ApiJson, ApiPath, ApiQuery},
        factories::response_factory::ResponseFactory,
    },
};
use axum::{
    extract::{Extension, State},
    response::Response,
};
use chrono::Datelike;
use uuid::Uuid;

pub async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Response, ApiError> {
    let routines = state.list_routines_use_case.execute(user.user_id).await?;
    Ok(ResponseFactory::ok(
        routines
            .into_iter()
            .map(RoutineResponse::from)
            .collect::<Vec<_>>(),
    ))
}
pub async fn save(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(request): ApiJson<RoutineRequest>,
) -> Result<Response, ApiError> {
    let routine = state
        .save_routine_use_case
        .execute(user.user_id, request.into_routine(id))
        .await?;
    Ok(ResponseFactory::ok(RoutineResponse::from(routine)))
}
pub async fn archive(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<Response, ApiError> {
    state
        .archive_routine_use_case
        .execute(user.user_id, id)
        .await?;
    Ok(ResponseFactory::no_content())
}
pub async fn week(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Response, ApiError> {
    Ok(ResponseFactory::ok(WeeklyScheduleResponse::from(
        state
            .get_weekly_schedule_use_case
            .execute(user.user_id)
            .await?,
    )))
}
pub async fn save_week(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(request): ApiJson<WeeklyScheduleRequest>,
) -> Result<Response, ApiError> {
    Ok(ResponseFactory::ok(WeeklyScheduleResponse::from(
        state
            .save_weekly_schedule_use_case
            .execute(user.user_id, request.into_days())
            .await?,
    )))
}
pub async fn daily(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiQuery(query): ApiQuery<TrainingDateQuery>,
) -> Result<Response, ApiError> {
    let date = query.parsed_date()?;
    let routine = state
        .get_daily_workout_use_case
        .execute(user.user_id, date)
        .await?;
    Ok(ResponseFactory::ok(DailyWorkoutResponse {
        date: query.date,
        weekday: date.weekday().number_from_monday(),
        routine: routine.map(Into::into),
    }))
}
