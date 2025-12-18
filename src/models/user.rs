use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub age: u8,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub age: u8,
}
