use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Macros {
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
    pub calories: f32,
}
