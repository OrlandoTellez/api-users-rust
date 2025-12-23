use crate::helpers::validate_gender::validate_gender;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub age: u8,
    pub gender: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, max = 30, message = "Only 1 and 30 characters are allowed."))]
    pub name: String,

    #[validate(length(min = 1, max = 30, message = "Only 1 and 30 characters are allowed."))]
    pub username: String,
    pub password: String,

    #[validate(range(min = 1, max = 100, message = "very high age"))]
    pub age: u8,

    #[validate(custom(function = "validate_gender"))]
    pub gender: String,
}
