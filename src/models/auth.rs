use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}
