use crate::{
    app_state::AppState,
    presentation::{
        authenticated_user::AuthenticatedUser,
        dto::{
            request::{
                daily_tracking::{
                    SessionUpdateRequest, StartSessionRequest, WaterGoalRequest, WaterRequest,
                },
                date::DateQuery,
            },
            response::daily_tracking::{DailyWaterResponse, SessionResponse},
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
use uuid::Uuid;

pub async fn start_session(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(input): ApiJson<StartSessionRequest>,
) -> Result<Response, ApiError> {
    let date = DateQuery { date: input.date }.parsed_date()?;
    Ok(ResponseFactory::ok(SessionResponse::from(
        state
            .workout_sessions
            .start(user.user_id, date, input.routine_id)
            .await?,
    )))
}
pub async fn save_session(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(input): ApiJson<SessionUpdateRequest>,
) -> Result<Response, ApiError> {
    Ok(ResponseFactory::ok(SessionResponse::from(
        state
            .workout_sessions
            .save(user.user_id, id, input.into())
            .await?,
    )))
}
pub async fn water(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiQuery(query): ApiQuery<DateQuery>,
) -> Result<Response, ApiError> {
    Ok(ResponseFactory::ok(DailyWaterResponse::from(
        state
            .hydration
            .daily(user.user_id, query.parsed_date()?)
            .await?,
    )))
}
pub async fn add_water(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(input): ApiJson<WaterRequest>,
) -> Result<Response, ApiError> {
    let date = DateQuery { date: input.date }.parsed_date()?;
    Ok(ResponseFactory::ok(DailyWaterResponse::from(
        state
            .hydration
            .add(user.user_id, date, input.id, input.amount_ml)
            .await?,
    )))
}
pub async fn remove_water(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<Response, ApiError> {
    state.hydration.remove(user.user_id, id).await?;
    Ok(ResponseFactory::no_content())
}
pub async fn water_goal(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(input): ApiJson<WaterGoalRequest>,
) -> Result<Response, ApiError> {
    let date = DateQuery { date: input.date }.parsed_date()?;
    Ok(ResponseFactory::ok(DailyWaterResponse::from(
        state
            .hydration
            .set_goal(user.user_id, date, input.goal_ml)
            .await?,
    )))
}
