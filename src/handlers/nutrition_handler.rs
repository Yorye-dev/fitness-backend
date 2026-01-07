use axum::{
    extract::{State,Json},
    response::IntoResponse,
};
use crate::services::Services;
use crate::dtos::sign_data_dto::SignInData;
use crate::factories::api_response_factory::HttpResponseFactory;



pub async fn get_daily_macros(
    State(service): State<NutritionService>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<DateQuery>,
) -> Result<HttpResponse, ApiError> {
    let date = query.date.unwrap_or_else(today);

    let result = service.get_daily_macros(user_id, date).await?;

    Ok(HttpResponseFactory::ok(result))
}
