use crate::presentation::errors::api_error::ApiError;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DailyNutritionQuery {
    pub date: String,
}
impl DailyNutritionQuery {
    pub fn parsed_date(&self) -> Result<NaiveDate, ApiError> {
        let date = NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")
            .map_err(|_| ApiError::Validation("date must be a valid YYYY-MM-DD date".into()))?;
        if self.date.len() != 10 || date.format("%Y-%m-%d").to_string() != self.date {
            return Err(ApiError::Validation(
                "date must be a valid YYYY-MM-DD date".into(),
            ));
        }
        Ok(date)
    }
}
