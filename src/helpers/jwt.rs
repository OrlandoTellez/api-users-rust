use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{config::constants::JWT_SECRET, helpers::errors::AppError, models::auth::Claim};

pub fn encode_jwt(username: String) -> Result<String, AppError> {
    // Configurar expiracion del token
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Valid timestmap")
        .timestamp();

    // crear el claim con la estrucutra ya hecha en models
    let claims: Claim = Claim {
        sub: username,
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };

    // crear jwt con encode
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError("Error creando jwt".to_string()))
}
