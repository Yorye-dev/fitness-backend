#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    InvalidRequest,
    ValidationError,

    Unauthorized,
    Forbidden,

    InvalidToken,

    UserNotFound,

    MealNotFound,
    NutritionGoalsNotFound,

    Conflict,

    InternalServerError,
}

impl ErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "INVALID_REQUEST",
            Self::ValidationError => "VALIDATION_ERROR",

            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",

            Self::InvalidToken => "INVALID_TOKEN",

            Self::UserNotFound => "USER_NOT_FOUND",

            Self::MealNotFound => "MEAL_NOT_FOUND",

            Self::NutritionGoalsNotFound => {
                "NUTRITION_GOALS_NOT_FOUND"
            }

            Self::Conflict => "CONFLICT",

            Self::InternalServerError => {
                "INTERNAL_SERVER_ERROR"
            }
        }
    }
}
