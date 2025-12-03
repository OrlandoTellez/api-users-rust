use crate::handlers::index;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/", get(index::hello_world))
}
