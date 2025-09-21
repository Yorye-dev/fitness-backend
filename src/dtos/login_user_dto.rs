use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginUserDto {
    pub username: String,
    pub password: String,
}
