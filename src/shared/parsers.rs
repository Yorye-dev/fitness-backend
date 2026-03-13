use uuid::Uuid;

pub fn parse_uuid(value: &str) -> Result<Uuid, String> {
    match Uuid::parse_str(value) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err(format!("Invalid UUID: {}", value)),
    }
}

pub fn convert_i32_at_f32(integer: i32) -> f32 {
    integer as f32
}
