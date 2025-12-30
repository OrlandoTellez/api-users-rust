use axum::Router;
use axum::routing::post;

use crate::handlers::auth;
use crate::states::db::Db;

pub fn routes() -> Router<Db> {
    Router::new().route("/auth/login", post(auth::login_user))
}
