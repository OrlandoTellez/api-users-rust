# API de Usuarios en Rust

Una API REST para la gestión de usuarios construida con Rust y el framework web Axum. Este proyecto implementa principios de clean arquitecture y proporciona autenticación basada en JWT, operaciones CRUD de usuarios y documentación OpenAPI automática.

## Características

- **Gestión de Usuarios**: Crear, leer, actualizar y eliminar usuarios
- **Autenticación**: Sistema de inicio de sesión basado en JWT con hash seguro de contraseñas
- **Validación**: Validación de entradas usando el crate `validator`
- **Documentación OpenAPI**: Documentación automática de la API con Swagger UI
- **Arquitectura Limpia**: Estructura de código organizada siguiendo principios de clean architecture

## Estructura del Proyecto

```
api-users-rust/
├── Cargo.toml
├── .env                    # Variables de entorno
├── .gitignore
└── src/
    ├── main.rs            # Punto de entrada, configuración del servidor
    ├── lib.rs             # Opcional: expone módulos públicos
    │
    ├── routes/            # Definición de rutas
    │   ├── mod.rs
    │   ├── users.rs
    │   └── auth.rs
    │
    ├── handlers/          # Controladores/Handlers (lógica de los endpoints)
    │   ├── mod.rs
    │   ├── users.rs
    │   └── auth.rs
    │
    ├── models/            # Estructuras de datos (DTOs, entidades)
    │   ├── mod.rs
    │   ├── user.rs
    │   └── response.rs
    │
    ├── services/          # Lógica de negocio
    │   ├── mod.rs
    │   └── user_service.rs
    │
    ├── helpers/           # Funciones auxiliares
    │   ├── mod.rs
    │   ├── jwt.rs
    │   └── validate_gender.rs
    │
    └── config/            # Configuración
        ├── mod.rs
        └── constants.rs
```

## Requisitos Previos

- Rust (última versión estable)
- Gestor de paquetes Cargo

## Instalación

1. Clonar el repositorio:

```bash
git clone https://github.com/OrlandoTellez/api-users-rust.git
cd api-users-rust
```

2. Instalar dependencias:

```bash
cargo build
```

3. Configurar variables de entorno:

```bash
cp .env.example .env
```

Edita el archivo `.env` y define tu secreto JWT:

```
JWT_SECRET=tu_clave_secreta_jwt_aqui
```

## Ejecución de la Aplicación

Inicia el servidor:

```bash
cargo run
```

El servidor se iniciará por defecto en `http://localhost:3000`. Puedes cambiar el puerto configurando la variable de entorno `PORT`.

## Endpoints de la API

### Autenticación

- `POST /auth/login` – Inicio de sesión de usuario

### Usuarios

- `GET /users` – Obtener todos los usuarios
- `POST /users` – Crear un nuevo usuario
- `GET /user/{id}` – Obtener un usuario por ID
- `PUT /user/{id}` – Actualizar un usuario por ID
- `DELETE /user/{id}` – Eliminar un usuario por ID

### Documentación

- `GET /docs` – Swagger UI para la documentación de la API
- `GET /api-docs/openapi.json` – Especificación OpenAPI en formato JSON

## Modelos de Datos

### Usuario

```json
{
  "id": 1,
  "name": "John Doe",
  "username": "johndoe",
  "password": "hashed_password",
  "age": 25,
  "gender": "male",
  "created_at": "2023-12-29T10:00:00"
}
```

### Solicitud de Creación de Usuario

```json
{
  "name": "John Doe",
  "username": "johndoe",
  "password": "securepassword",
  "age": 25,
  "gender": "male"
}
```

### Solicitud de Login

```json
{
  "username": "johndoe",
  "password": "securepassword"
}
```

### Respuesta de Login

```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer"
}
```

## Reglas de Validación

- **Nombre**: 1–30 caracteres
- **Usuario**: 1–30 caracteres
- **Contraseña**: Mínimo 6 caracteres
- **Edad**: 1–100 años
- **Género**: Debe ser `"male"`, `"female"` o `"other"`

## Dependencias

- **axum**: Framework web
- **serde**: Serialización/deserialización
- **tokio**: Runtime asíncrono
- **chrono**: Manejo de fechas y tiempo
- **validator**: Validación de entradas
- **jsonwebtoken**: Manejo de tokens JWT
- **bcrypt**: Hash de contraseñas
- **utoipa**: Generación de documentación OpenAPI
- **utoipa-swagger-ui**: Integración de Swagger UI

## Notas de Arquitectura

Este proyecto sigue los principios de arquitectura limpia:

- **Routes**: Definen los endpoints y métodos HTTP
- **Handlers**: Contienen la lógica de los endpoints y el manejo de respuestas HTTP
- **Services**: Implementan la lógica de negocio
- **Models**: Definen estructuras de datos y DTOs
- **Helpers**: Proveen funciones utilitarias (JWT, validaciones, etc.)
- **Config**: Gestiona la configuración de la aplicación

La aplicación utiliza almacenamiento en memoria por simplicidad. En un entorno de producción, deberías reemplazarlo por una base de datos real.

## Seguridad

- Las contraseñas se hashean usando bcrypt
- Los tokens JWT se utilizan para la autenticación
- La validación de entradas previene datos maliciosos

## Contribuciones

1. Haz un fork del repositorio
2. Crea una rama de feature
3. Realiza tus cambios
4. Ejecuta tests y formateo
5. Envía un pull request

