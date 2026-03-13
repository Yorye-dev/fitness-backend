use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Debug, Type, Clone)]
#[sqlx(type_name = "sex", rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
}
