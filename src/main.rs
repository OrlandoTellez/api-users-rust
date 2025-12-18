// aqui vamos a hacer algo muy basico con axum, crear un servidor web
mod models;
mod handlers;
mod routes;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

const PORT: &str = "3000";

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    age: u8,
}

#[tokio::main]
async fn main() {
    let app = routes::create_routes();

    let url = format!("0.0.0.0:{}", PORT);

    // ojo qui es importante saber el concepto de pasar propiedad, por eso el & es importante, porque no pasamos la propiedad, sino solo prestamos el valor.
    let listener = TcpListener::bind(&url).await.unwrap();

    println!("Server listener in http://{}", &url);

    axum::serve(listener, app).await.unwrap();
}

// Nota: es aceraca deel unwrap, resulta que en producción no es tan recomendable porque practicamente le dice al programa que si falla haga un panic, hay mejores formas de manejar un error.
