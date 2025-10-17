use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Debug)]
#[sqlx(type_name = "goal", rename_all = "snake_case")]
// agregar Type
pub enum Goal{
    LoseWeigh,
    Maintain,
    GainMuscle,
}
