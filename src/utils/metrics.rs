use crate::utils::parsers;

pub fn calculate_tmb(weight: f32, height:i32, age:i32) -> f32 {
    10.00 * weight + 6.25 * parsers::convert_i32_at_f32(height) - 5.00 * parsers::convert_i32_at_f32(age) + 5.00
}
