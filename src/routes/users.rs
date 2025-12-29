use crate::{handlers::users, states::app_state::AppState};
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(users::get_users).post(users::create_user))
        .route(
            "/user/{id}",
            get(users::get_user_by_id)
                .put(users::update_user)
                .delete(users::delete_user),
        )
}
