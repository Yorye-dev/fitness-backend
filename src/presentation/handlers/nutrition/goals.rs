use crate::{
    app_state::AppState,
    presentation::{
        authenticated_user::AuthenticatedUser,
        dto::{
            request::nutrition::update_goals_request::UpdateGoalsRequest,
            response::nutrition::nutrition_goals_response::NutritionGoalsResponse,
        },
        errors::api_error::ApiError,
        extractors::ApiJson,
        factories::response_factory::ResponseFactory,
    },
};
use axum::{
    extract::{Extension, State},
    response::Response,
};
pub async fn get(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Response, ApiError> {
    Ok(ResponseFactory::ok(NutritionGoalsResponse::from(
        state.get_goals_use_case.execute(user.user_id).await?,
    )))
}
pub async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(request): ApiJson<UpdateGoalsRequest>,
) -> Result<Response, ApiError> {
    let goals = state
        .update_goals_use_case
        .execute(user.user_id, request.into())
        .await?;
    Ok(ResponseFactory::ok(NutritionGoalsResponse::from(goals)))
}
