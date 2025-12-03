use std::vec;

// aqui vamos a hacer algo muy basico con axum, crear un servidor web
use axum::{Router, response::Json, routing::get};
use serde::{Deserialize, Serialize};
// use serde_json::{Value, json};
use tokio::net::TcpListener;

const PORT: &str = "3000";

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    age: u8,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(list_users));

    let url = format!("0.0.0.0:{}", PORT);

    // ojo qui es importante saber elconcepto de pasar propiedad, por eso el & es importante, porque no pasamos la propiedad, sino solo prestamos el valor.
    let listener = TcpListener::bind(&url).await.unwrap();

    println!("Server listener in http://{}", &url);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "hello world"
}

// async fn list_userrs() -> Json<Value> {
//     let new_user = json!({
//         "name": String::from("Orlando"),
//         "age": 20,
//     });

//     Json(new_user)
// }

async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            name: String::from("Orlando"),
            age: 20,
        },
        User {
            name: String::from("David"),
            age: 21,
        },
    ];

    Json(users)
}

// Nota: es aceraca deel unwrap, resulta que en producción no es tan recomendable porque practicamente le dice al programa que si falla haga un panic, hay mejores formas de manejar un error.
