// hay que importarlo en el archivo main.rs para que pueda ser accesible en toda la app
// en esta carpeta routes se registran todas las rutas que tiene el servidor
pub mod index;
pub mod users;
use axum::Router;

pub fn create_routes() -> Router {
    Router::new().merge(index::routes()).merge(users::routes())
}
