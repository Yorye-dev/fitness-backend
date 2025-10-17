use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Debug)]
#[sqlx(type_name = "sex", rename_all = "snake_case")]

pub enum Sex {
    Male,
    Female,
}
