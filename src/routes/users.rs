use crate::{handlers::users, states::db::Db};
use axum::{Router, routing::get};

pub fn routes() -> Router<Db> {
    Router::new()
        .route("/users", get(users::get_users).post(users::create_user))
        .route(
            "/user/{id}",
            get(users::get_user_by_id)
                .patch(users::update_user)
                .delete(users::delete_user),
        )
}
