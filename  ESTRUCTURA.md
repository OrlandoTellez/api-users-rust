Clean arquitecture pero enfocada en Rust con Axum:

Ejemplo de la estructura de carpetas:

api-users-rust/
├── Cargo.toml
├── .env                    # Variables de entorno
├── .gitignore
└── src/
    ├── main.rs            # Entry point, configuración del servidor
    ├── lib.rs             # Opcional: para exponer módulos públicos
    │
    ├── routes/            # Definición de rutas
    │   ├── mod.rs
    │   ├── users.rs
    │   └── auth.rs
    │
    ├── handlers/          # Controladores/Handlers (lógica de endpoints)
    │   ├── mod.rs
    │   ├── users.rs
    │   └── auth.rs
    │
    ├── models/            # Estructuras de datos (DTOs, entities)
    │   ├── mod.rs
    │   ├── user.rs
    │   └── response.rs
    │
    ├── services/          # Lógica de negocio
    │   ├── mod.rs
    │   └── user_service.rs
    │
    ├── repositories/      # Acceso a datos (DB, cache, etc)
    │   ├── mod.rs
    │   └── user_repository.rs
    │
    ├── middleware/        # Middleware (auth, logging, etc)
    │   ├── mod.rs
    │   └── auth.rs
    │
    ├── utils/             # Utilidades
    │   ├── mod.rs
    │   └── validation.rs
    │
    └── config/            # Configuración
        ├── mod.rs
        └── database.rs