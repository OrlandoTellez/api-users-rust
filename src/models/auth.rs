use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}
