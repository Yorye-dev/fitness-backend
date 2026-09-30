use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NutritionBasis {
    Per100g,
    PerUnit,
}

impl NutritionBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Per100g => "per_100g",
            Self::PerUnit => "per_unit",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Meal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub nutrition_basis: NutritionBasis,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}
