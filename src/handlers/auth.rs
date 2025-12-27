use axum::{Json, extract::State};

use crate::{
    helpers::{errors::AppError, success_response::success_response},
    models::{
        auth::{AuthResponse, LoginPayload},
        response::ApiResponse,
    },
    services::auth_service::AuthService,
    states::app_state::AppState,
};

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let login_user = AuthService::login_user(&state, payload).await?;

    Ok(Json(success_response(login_user, "Login exitoso")))
}
