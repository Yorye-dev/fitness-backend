use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[sqlx(type_name = "unit_type", rename_all = "snake_case")]
pub enum UnitType {
    Gram,
    Piece,
    Ml,
    Portion,
}

