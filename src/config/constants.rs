use std::env;

use once_cell::sync::Lazy;

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "clave_super_secreta_jaja_usar_la_de_entorno_env".to_string())
});
