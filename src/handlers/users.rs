use crate::helpers::errors::AppError;
use crate::helpers::success_response::success_response;
use crate::models::response::ApiResponse;
use crate::models::user::{CreateUser, User};
use crate::services::user_service::UserService;
use crate::states::app_state::AppState;

use axum::{Json, extract::State};

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<User>>>, AppError> {
    let users = UserService::get_users(&state).await?;

    Ok(Json(success_response(users, "User list received")))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    let user = UserService::create_user(&state, payload).await?;

    Ok(Json(success_response(user, "Usuario creado exitosamente")))
}
