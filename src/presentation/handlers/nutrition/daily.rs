use crate::{
    app_state::AppState,
    presentation::{
        authenticated_user::AuthenticatedUser,
        dto::{
            request::nutrition::daily_nutrition_query::DailyNutritionQuery,
            response::nutrition::daily_nutrition_response::DailyNutritionResponse,
        },
        errors::api_error::ApiError,
        extractors::ApiQuery,
        factories::response_factory::ResponseFactory,
    },
};
use axum::{
    extract::{Extension, State},
    response::Response,
};
pub async fn get_daily(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    ApiQuery(query): ApiQuery<DailyNutritionQuery>,
) -> Result<Response, ApiError> {
    let day = state
        .get_daily_nutrition_use_case
        .execute(user.user_id, query.parsed_date()?)
        .await?;
    Ok(ResponseFactory::ok(DailyNutritionResponse::from(day)))
}
