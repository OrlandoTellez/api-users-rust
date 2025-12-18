use crate::handlers::users;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/users", get(users::get_users))
}
