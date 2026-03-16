use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInData {
    pub username: String,
    pub password: String,
}

impl SignInData {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.username.is_empty() {
            errors.push("username cannot be empty".to_string());
        }

        if self.password.is_empty() {
            errors.push("password cannot be empty".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
