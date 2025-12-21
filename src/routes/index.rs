use crate::{handlers::index, states::app_state::AppState};
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(index::hello_world))
}
