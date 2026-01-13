use sqlx::Type;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Type, Clone)]
#[sqlx(type_name = "sex", rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
}
