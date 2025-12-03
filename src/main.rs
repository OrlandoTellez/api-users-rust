// aqui vamos a hacer algo muy basico con axum, crear un servidor web
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// Nota: es aceraca deel unwrap, resulta que en producción no es tan recomendable porque practicamente le dice al programa que si falla haga un panic, hay mejores formas de manejar un error.
