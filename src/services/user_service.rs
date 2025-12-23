use chrono::Utc;
use validator::Validate;

use crate::{
    helpers::errors::AppError,
    models::user::{CreateUser, User},
    states::app_state::AppState,
};

pub struct UserService;

impl UserService {
    pub async fn get_users(state: &AppState) -> Result<Vec<User>, AppError> {
        let users = state
            .lock()
            .map_err(|_| AppError::InternalServerError("Internal server error".to_string()))?;

        Ok(users.clone())
    }

    pub async fn create_user(state: &AppState, payload: CreateUser) -> Result<User, AppError> {
        payload.validate().map_err(AppError::ValidationError)?;

        let mut users = state
            .lock()
            .map_err(|_| AppError::InternalServerError("Internal server error".to_string()))?;

        let new_user: User = User {
            id: users.len() as u32 + 1,
            name: payload.name,
            username: payload.username,
            age: payload.age,
            password: payload.password, // aun hay que hashear la contraseña y no devolverla xd
            gender: payload.gender,
            created_at: Utc::now().naive_utc(),
        };

        users.push(new_user.clone());

        Ok(new_user)
    }
}
