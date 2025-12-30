// hay que importarlo en el archivo main.rs para que pueda ser accesible en toda la app
// en esta carpeta routes se registran todas las rutas que tiene el servidor
pub mod auth;
pub mod index;
pub mod users;

use crate::openapi::ApiDoc;
use crate::states::db::Db;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_routes() -> Router<Db> {
    Router::new()
        .merge(index::routes())
        .merge(users::routes())
        .merge(auth::routes())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
