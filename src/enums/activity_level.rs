use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ActivityLevel {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtraActive,
}



impl ActivityLevel {
    pub fn multiplier(&self) -> f32 {
        match self {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::LightlyActive => 1.375,
            ActivityLevel::ModeratelyActive => 1.55,
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtraActive => 1.9,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            ActivityLevel::Sedentary => "Sedentary".to_string(),
            ActivityLevel::LightlyActive => "LightlActive".to_string(),
            ActivityLevel::ModeratelyActive => "ModeratelyActive".to_string(),
            ActivityLevel::VeryActive => "VeryActive".to_string(),
            ActivityLevel::ExtraActive => "ExtraActive".to_string(),
        }
    }
}
