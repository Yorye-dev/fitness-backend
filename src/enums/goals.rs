use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Debug, Type)]
#[sqlx(type_name = "goal", rename_all = "snake_case")]
pub enum Goal {
    LoseWeight,
    Maintain,
    GainMuscle,
}
