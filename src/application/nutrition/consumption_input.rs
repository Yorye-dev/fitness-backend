use crate::domain::{errors::DomainError, nutrition::consumption::ConsumptionQuantity};

pub struct ConsumptionQuantityInput {
    pub quantity_grams: Option<f64>,
    pub portion_count: Option<f64>,
    pub portion_grams: Option<f64>,
}

impl ConsumptionQuantityInput {
    pub fn resolve(self) -> Result<ConsumptionQuantity, DomainError> {
        match (self.quantity_grams, self.portion_count, self.portion_grams) {
            (Some(grams), None, None) if valid_amount(grams) => Ok(ConsumptionQuantity {
                quantity_grams: Some(grams),
                portion_count: None,
                portion_grams: None,
            }),
            (None, Some(count), Some(grams)) if valid_amount(count) && valid_amount(grams) => {
                // Match PostgreSQL NUMERIC(10,3); the server owns the conversion to grams.
                let count_millis = (count * 1000.0).round() as u64;
                let grams_millis = (grams * 1000.0).round() as u64;
                let total = ((count_millis * grams_millis + 500) / 1000) as f64 / 1000.0;
                if !valid_amount(total) {
                    return Err(DomainError::Validation(
                        "total quantity must be between 0.001 and 1000000 grams".into(),
                    ));
                }
                Ok(ConsumptionQuantity {
                    quantity_grams: Some(total),
                    portion_count: Some(count),
                    portion_grams: Some(grams),
                })
            }
            (None, Some(count), None) if valid_amount(count) => Ok(ConsumptionQuantity {
                quantity_grams: None, portion_count: Some(count), portion_grams: None,
            }),
            _ => Err(DomainError::Validation(
                "provide grams, weighted portions, or only portion_count for a unit-based food; values must be between 0.001 and 1000000 with at most 3 decimals".into(),
            )),
        }
    }
}

fn valid_amount(value: f64) -> bool {
    value.is_finite()
        && (0.001..=1_000_000.0).contains(&value)
        && (value * 1000.0 - (value * 1000.0).round()).abs() < 0.000001
}
