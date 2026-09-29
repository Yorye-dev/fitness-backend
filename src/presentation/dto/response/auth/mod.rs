use crate::application::auth::login::AuthTokens;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthTokensResponse {
    pub access_token: String,
    pub refresh_token: String,
}
impl From<AuthTokens> for AuthTokensResponse {
    fn from(tokens: AuthTokens) -> Self {
        Self {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        }
    }
}
#[derive(Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
}
