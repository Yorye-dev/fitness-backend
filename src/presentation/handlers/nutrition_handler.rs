use axum::{
    extract::{Query, State, Extension},
    response::IntoResponse,
};
use serde::Deserialize;
use crate::services::Services;
use crate::auth::claims::Claims;

#[derive(Deserialize)]
pub struct NutritionQuery {
    date: Option<String>,
}

pub async fn daily_macros_handler(
    Query(query): Query<NutritionQuery>,
    Extension(_claims): Extension<Claims>,
    State(_services): State<Services>,
) -> impl IntoResponse {
    let _date = if let Some(d) = query.date {
        chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap()
    } else {
        chrono::Utc::now().date_naive()
    };
}
