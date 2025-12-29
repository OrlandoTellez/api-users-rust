use utoipa::OpenApi;

use crate::handlers::auth;
use crate::handlers::index;
use crate::handlers::users;
use crate::models::response::ApiResponse;
use crate::models::user::{CreateUser, UpdateUser, User};

#[derive(OpenApi)]
#[openapi(
    paths(
        index::hello_world,
        users::get_users,
        users::get_user_by_id,
        users::create_user,
        users::update_user,
        users::delete_user,
        auth::login_user,
    ),
    components(
        schemas(User, CreateUser, UpdateUser, ApiResponse<User>, ApiResponse<Vec<User>>)
    ),
    tags(
        (name = "Users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;
