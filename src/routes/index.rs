use crate::{handlers::index, states::db::Db};
use axum::{Router, routing::get};

pub fn routes() -> Router<Db> {
    Router::new().route("/", get(index::hello_world))
}
