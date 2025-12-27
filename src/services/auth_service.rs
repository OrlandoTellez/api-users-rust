use bcrypt::verify;

use crate::{
    helpers::{errors::AppError, jwt::encode_jwt},
    models::auth::{AuthResponse, LoginPayload},
    states::app_state::AppState,
};

pub struct AuthService;

impl AuthService {
    pub async fn login_user(
        state: &AppState,
        payload: LoginPayload,
    ) -> Result<AuthResponse, AppError> {
        // obtener usuarios del estado global
        let users = state
            .lock()
            .map_err(|_| AppError::InternalServerError("Internal server error".to_string()))?;

        // encontrar el usuario que se quiere logear
        let user = users
            .iter()
            .find(|u| u.username == payload.username)
            .ok_or(AppError::NotFound("Usuario no encontrado".to_string()))?;

        // verificar las contraseñas
        let valid_password = verify(payload.password, &user.password)
            .map_err(|_| AppError::InternalServerError("Error verify password".to_string()))?;

        if !valid_password {
            return Err(AppError::NotFound("Invalid credentials".to_string()));
        }

        let token = encode_jwt(user.username.clone())?;

        Ok(AuthResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
        })
    }
}
