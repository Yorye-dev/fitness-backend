#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityLevel {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtraActive,
}

impl ActivityLevel {
    pub fn multiplier(self) -> f32 {
        match self {
            Self::Sedentary => 1.2,
            Self::LightlyActive => 1.375,
            Self::ModeratelyActive => 1.55,
            Self::VeryActive => 1.725,
            Self::ExtraActive => 1.9,
        }
    }
}
