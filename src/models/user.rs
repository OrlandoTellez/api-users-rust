use crate::helpers::validate_gender::validate_gender;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Serialize, ToSchema)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub age: u8,
    pub gender: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateUser {
    #[validate(length(min = 1, max = 30, message = "Only 1 and 30 characters are allowed."))]
    pub name: String,

    #[validate(length(min = 1, max = 30, message = "Only 1 and 30 characters are allowed."))]
    pub username: String,

    #[validate(length(min = 6, message = "The password must be at least 6 characters long."))]
    pub password: String,

    #[validate(range(min = 1, max = 100, message = "very high age"))]
    pub age: u8,

    #[validate(custom(function = "validate_gender"))]
    pub gender: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateUser {
    #[validate(length(min = 1, max = 30, message = "Only 1 and 30 characters are allowed."))]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 30, message = "Only 1 and 30 characters are allowed."))]
    pub username: Option<String>,

    #[validate(length(min = 6, message = "The password must be at least 6 characters long."))]
    pub password: Option<String>,

    #[validate(range(min = 1, max = 100, message = "very high age"))]
    pub age: Option<u8>,

    #[validate(custom(function = "validate_gender"))]
    pub gender: Option<String>,
}
