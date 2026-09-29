use crate::{
    app_state::AppState,
    presentation::{
        authenticated_user::AuthenticatedUser,
        dto::{
            request::nutrition::meal_request::{MealListQuery, MealRequest},
            response::nutrition::meal_response::MealResponse,
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
pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiJson(request): ApiJson<MealRequest>,
) -> Result<Response, ApiError> {
    let meal = state
        .create_meal_use_case
        .execute(user.user_id, request.into())
        .await?;
    Ok(ResponseFactory::created(MealResponse::from(meal)))
}
pub async fn get(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(meal_id): ApiPath<Uuid>,
) -> Result<Response, ApiError> {
    Ok(ResponseFactory::ok(MealResponse::from(
        state
            .get_meal_use_case
            .execute(user.user_id, meal_id)
            .await?,
    )))
}
pub async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiQuery(query): ApiQuery<MealListQuery>,
) -> Result<Response, ApiError> {
    let (meals, total) = state
        .list_meals_use_case
        .execute(user.user_id, query.page, query.per_page)
        .await?;
    Ok(ResponseFactory::paginated(
        meals.into_iter().map(MealResponse::from).collect(),
        query.page,
        query.per_page,
        total as u64,
    ))
}
pub async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(meal_id): ApiPath<Uuid>,
    ApiJson(request): ApiJson<MealRequest>,
) -> Result<Response, ApiError> {
    let meal = state
        .update_meal_use_case
        .execute(user.user_id, meal_id, request.into())
        .await?;
    Ok(ResponseFactory::ok(MealResponse::from(meal)))
}
pub async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiPath(meal_id): ApiPath<Uuid>,
) -> Result<Response, ApiError> {
    state
        .delete_meal_use_case
        .execute(user.user_id, meal_id)
        .await?;
    Ok(ResponseFactory::no_content())
}
