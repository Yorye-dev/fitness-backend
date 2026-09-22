use serde::Deserialize;

use crate::application::auth::login::LoginInput;

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

impl SignInRequest {
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

impl From<SignInRequest> for LoginInput {
    fn from(request: SignInRequest) -> Self {
        Self {
            username: request.username,
            password: request.password,
        }
    }
}
