use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChangePasswordDto {
    pub current_password: String,
    pub new_password: String,
}

impl ChangePasswordDto {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.current_password.is_empty() {
            errors.push("current_password cannot be empty".to_string());
        }

        if self.new_password.is_empty() {
            errors.push("new_password cannot be empty".to_string());
        }
        if self.new_password.len() < 8 {
            errors.push("new_password must be at least 8 characters".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
