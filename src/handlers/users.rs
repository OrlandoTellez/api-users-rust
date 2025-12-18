use crate::models::user::{CreateUser, User};
use axum::Json;
use chrono::Utc;

pub async fn get_users() -> Json<Vec<User>> {
    let users = vec![User {
        id: 1,
        name: String::from("Orlando"),
        username: String::from("orlandotellez12"),
        password: String::from("123456"),
        age: 20,
        created_at: Utc::now().naive_utc(),
    }];

    Json(users)
}

//pub async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {}
