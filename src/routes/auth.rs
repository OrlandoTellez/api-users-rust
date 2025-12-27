use axum::Router;
use axum::routing::post;

use crate::handlers::auth;
use crate::states::app_state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/auth/login", post(auth::login_user))
}
