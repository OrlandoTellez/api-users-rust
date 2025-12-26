use std::env;

use once_cell::sync::Lazy;

pub static JWT_SECRET: Lazy<String> =
    Lazy::new(|| env::var("JWT_SECRET").expect("Variable de entorno requerida"));
