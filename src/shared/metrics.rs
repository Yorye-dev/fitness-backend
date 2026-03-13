use crate::enums::activity_level;
use crate::shared::parsers;

pub fn calculate_tdee(weight: f32, height: i32, age: i32) -> f32 {
    let tmb = calculate_bmr(weight, height, age);
    let activity = activity_level::ActivityLevel::VeryActive;
    tmb * activity.multiplier()
}

fn calculate_bmr(weight: f32, height: i32, age: i32) -> f32 {
    10.00 * weight + 6.25 * parsers::convert_i32_at_f32(height)
        - 5.00 * parsers::convert_i32_at_f32(age)
        + 5.00
}
