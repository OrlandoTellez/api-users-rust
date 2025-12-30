// aqui vamos a hacer algo muy basico con axum, crear un servidor web
mod config;
mod handlers;
mod helpers;
mod models;
mod openapi;
mod routes;
mod services;
mod states;

use crate::config::constants::DATABASE_URL;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&DATABASE_URL)
        .await
        .expect("Failed to connect to database");

    let app = routes::create_routes().with_state(db);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server listener on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// Nota: es aceraca deel unwrap, resulta que en producción no es tan recomendable porque practicamente le dice al programa que si falla haga un panic, hay mejores formas de manejar un error.
