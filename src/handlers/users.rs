use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    name: String,
    age: u8,
}

pub async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            name: String::from("Orlando"),
            age: 20,
        },
        User {
            name: String::from("David"),
            age: 21,
        },
    ];

    Json(users)
}
