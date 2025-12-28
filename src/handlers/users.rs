use crate::helpers::errors::AppError;
use crate::helpers::success_response::success_response;
use crate::models::response::ApiResponse;
use crate::models::user::{CreateUser, UpdateUser, User};
use crate::services::user_service::UserService;
use crate::states::app_state::AppState;

use axum::extract::Path;
use axum::{Json, extract::State};

#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    responses(
        (status = 200, description = "Lista de usuarios", body = ApiResponse<Vec<User>>)
    )
)]
pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<User>>>, AppError> {
    let users: Vec<User> = UserService::get_users(&state).await?;

    let response: ApiResponse<Vec<User>> = success_response(users, "User list received");

    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/users",
    tag = "Users",
    request_body = CreateUser,
    responses(
        (status = 200, description = "Usuario creado", body = ApiResponse<User>)
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    let user: User = UserService::create_user(&state, payload).await?;

    let response: ApiResponse<User> = success_response(user, "Usuario creado exitosamente");

    Ok(Json(response))
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = u32, Path, description = "ID del usuario")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Usuario actualizado", body = ApiResponse<User>),
        (status = 404, description = "Usuario no encontrado")
    )
)]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    let user: User = UserService::update_user(&state, id, payload).await?;

    let response: ApiResponse<User> = success_response(user, "user updated successfully");

    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = u32, Path, description = "ID del usuario")
    ),
    responses(
        (status = 200, description = "Usuario eliminado"),
        (status = 404, description = "Usuario no encontrado")
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    UserService::delete_user(&state, id).await?;

    let response: ApiResponse<()> = success_response((), "user deleted successfully");

    Ok(Json(response))
}
