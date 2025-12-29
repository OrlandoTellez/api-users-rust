use bcrypt::{DEFAULT_COST, hash};
use chrono::Utc;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    helpers::errors::AppError,
    models::user::{CreateUser, UpdateUser, User},
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

    pub async fn get_user_by_id(state: &AppState, id: u32) -> Result<User, AppError> {
        let users = state
            .lock()
            .map_err(|_| AppError::InternalServerError("Internal server error".to_string()))?;

        let user_by_id = users
            .iter()
            .find(|u| u.id == id)
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user_by_id.clone())
    }

    pub async fn create_user(state: &AppState, payload: CreateUser) -> Result<User, AppError> {
        payload.validate().map_err(AppError::ValidationError)?;

        let mut users = state
            .lock()
            .map_err(|_| AppError::InternalServerError("Internal server error".to_string()))?;

        let hashed_password: String = hash(payload.password, DEFAULT_COST).unwrap();

        if users.iter().any(|u| u.username == payload.username) {
            let mut errors = ValidationErrors::new();
            let mut error = ValidationError::new("unique");

            error.message = Some("El usuario ya existe".into());

            errors.add("username", error);

            return Err(AppError::ValidationError(errors));
        }

        let new_user: User = User {
            id: users.len() as u32 + 1,
            name: payload.name,
            username: payload.username,
            age: payload.age,
            password: hashed_password,
            gender: payload.gender,
            created_at: Utc::now().naive_utc(),
        };

        users.push(new_user.clone());

        Ok(new_user)
    }

    pub async fn update_user(
        state: &AppState,
        id: u32,
        payload: UpdateUser,
    ) -> Result<User, AppError> {
        payload.validate().map_err(AppError::ValidationError)?;

        let mut users = state
            .lock()
            .map_err(|_| AppError::InternalServerError("Internal server error".to_string()))?;

        let user = users
            .iter_mut()
            .find(|u| u.id == id)
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if let Some(name) = payload.name {
            user.name = name;
        }

        if let Some(username) = payload.username {
            user.username = username;
        }

        if let Some(age) = payload.age {
            user.age = age;
        }

        if let Some(gender) = payload.gender {
            user.gender = gender;
        }

        if let Some(password) = payload.password {
            user.password = hash(password, DEFAULT_COST).unwrap();
        }

        Ok(user.clone())
    }

    pub async fn delete_user(state: &AppState, id: u32) -> Result<(), AppError> {
        let mut users = state
            .lock()
            .map_err(|_| AppError::InternalServerError("Internal server error".to_string()))?;

        let user = users
            .iter()
            .position(|u| u.id == id)
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        users.remove(user);

        Ok(())
    }
}
