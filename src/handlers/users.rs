use crate::helpers::errors::AppError;
use crate::helpers::success_response::success_response;
use crate::models::response::ApiResponse;
use crate::models::user::{CreateUser, UpdateUser, User};
use crate::services::user_service::UserService;
use crate::states::db::Db;

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
pub async fn get_users(State(db): State<Db>) -> Result<Json<ApiResponse<Vec<User>>>, AppError> {
    let users: Vec<User> = UserService::get_users(&db).await.unwrap();

    let response: ApiResponse<Vec<User>> = success_response(users, "User list received");

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = u32, Path, description = "ID del usuario")
    ),
    request_body = User,
    responses(
        (status = 200, description = "Usuario encontrado", body = ApiResponse<User>),
        (status = 404, description = "Usuario no encontrado")
    )
)]
pub async fn get_user_by_id(
    State(bd): State<Db>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    let user_by_id: User = UserService::get_user_by_id(&bd, id).await.unwrap();

    let response: ApiResponse<User> = success_response(user_by_id, "User received");

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
    State(bd): State<Db>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    let user: User = UserService::create_user(&bd, payload).await?;

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
    State(bd): State<Db>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    let user_option = UserService::update_user(&bd, id, payload).await?;

    match user_option {
        Some(user) => {
            let response: ApiResponse<User> = success_response(user, "user updated successfully");
            Ok(Json(response))
        }
        None => Err(AppError::NotFound("User not found".into())),
    }
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
    State(bd): State<Db>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    UserService::delete_user(&bd, id).await?;

    let response: ApiResponse<()> = success_response((), "user deleted successfully");

    Ok(Json(response))
}
