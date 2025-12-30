use axum::{Json, extract::State};

use crate::{
    helpers::{errors::AppError, success_response::success_response},
    models::{
        auth::{AuthResponse, LoginPayload},
        response::ApiResponse,
    },
    services::auth_service::AuthService,
    states::db::Db,
};

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Inicio de sesión", body = ApiResponse<AuthResponse>)
    )
)]
pub async fn login_user(
    State(state): State<Db>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let login_user = AuthService::login_user(&state, payload).await?;

    Ok(Json(success_response(login_user, "Login exitoso")))
}
