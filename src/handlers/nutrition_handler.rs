use axum::extract::Query;
use serde::Deserialize;

use crate::services::Services;
use crate::auth::claims::Claims;

#[derive(Deserialize)]
struct NutritionQuery {
    date: Option<String>, // formato "YYYY-MM-DD"
}

pub async fn daily_macros_handler(
    Query(query): Query<NutritionQuery>,
    Extension(claims): Extension<Claims>,
    State(services): State<Services> ,
) -> impl IntoResponse {
    let date = if let Some(d) = query.date {
        // parsear string a chrono::NaiveDate
        chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap()
    } else {
        Utc::now().date_naive()
    };

    // llamar al servicio.
}
