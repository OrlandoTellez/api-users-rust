use bcrypt::verify;

use crate::{
    helpers::{errors::AppError, jwt::encode_jwt},
    models::auth::{AuthResponse, LoginPayload},
    states::db::Db,
};

pub struct AuthService;

impl AuthService {
    pub async fn login_user(db: &Db, payload: LoginPayload) -> Result<AuthResponse, AppError> {
        let user = sqlx::query!(
            r#"
            SELECT id, username, password
            FROM users
            WHERE username = $1
            "#,
            payload.username
        )
        .fetch_optional(db)
        .await?;

        let user = match user {
            Some(u) => u,
            None => {
                // ⚠️ no revelar si el usuario existe o no
                return Err(AppError::NotFound("Invalid credentials".into()));
            }
        };

        let valid_password = verify(&payload.password, &user.password)
            .map_err(|_| AppError::InternalServerError("Password verification failed".into()))?;

        if !valid_password {
            return Err(AppError::NotFound("Invalid credentials".into()));
        }

        let token = encode_jwt(user.username)?;

        Ok(AuthResponse {
            access_token: token,
            token_type: "Bearer".into(),
        })
    }
}
