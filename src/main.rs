// aqui vamos a hacer algo muy basico con axum, crear un servidor web
mod handlers;
mod helpers;
mod models;
mod routes;
mod services;
mod states;

use crate::{models::user::User, states::app_state::AppState};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

const PORT: &str = "3000";

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(Mutex::new(Vec::<User>::new()));

    let app = routes::create_routes().with_state(state);

    let url = format!("0.0.0.0:{}", PORT);

    // ojo qui es importante saber el concepto de pasar propiedad, por eso el & es importante, porque no pasamos la propiedad, sino solo prestamos el valor.
    let listener = TcpListener::bind(&url).await.unwrap();

    println!("Server listener in http://{}", &url);

    axum::serve(listener, app).await.unwrap();
}

// Nota: es aceraca deel unwrap, resulta que en producción no es tan recomendable porque practicamente le dice al programa que si falla haga un panic, hay mejores formas de manejar un error.
