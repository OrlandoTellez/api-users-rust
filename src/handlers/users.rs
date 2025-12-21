use crate::models::user::{CreateUser, User};
use crate::states::app_state::AppState;
use axum::{Json, extract::State};
use chrono::Utc;

pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.lock().unwrap();

    Json(users.clone())
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let mut users = state.lock().unwrap();

    let user = User {
        id: 1,
        name: payload.name,
        username: payload.username,
        password: payload.password,
        age: payload.age,
        created_at: Utc::now().naive_utc(),
    };

    users.push(user.clone());

    Json(user)
}
