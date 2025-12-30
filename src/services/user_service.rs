use bcrypt::{DEFAULT_COST, hash};
use validator::Validate;

use crate::{
    helpers::errors::AppError,
    models::user::{CreateUser, UpdateUser, User},
    states::db::Db,
};

pub struct UserService;

impl UserService {
    pub async fn get_users(db: &Db) -> Result<Vec<User>, AppError> {
        let users: Vec<User> = sqlx::query_as!(
            User,
            r#"
                SELECT id, name, password, username, age, gender as "gender: _", created_at
                FROM users
                ORDER BY id
            "#
        )
        .fetch_all(db)
        .await?;

        Ok(users)
    }

    pub async fn get_user_by_id(bd: &Db, id: i32) -> Result<User, AppError> {
        let user: User = sqlx::query_as!(
            User,
            r#"
                SELECT id, name, password, username, age, gender as "gender: _", created_at
                FROM users
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(bd)
        .await?;

        Ok(user)
    }

    pub async fn create_user(db: &Db, payload: CreateUser) -> Result<User, AppError> {
        payload.validate().map_err(AppError::ValidationError)?;

        let hashed_password: String = hash(payload.password, DEFAULT_COST).unwrap();
        let user: User = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (name, username, password, age, gender)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, name, username, password, age, gender as "gender: _", created_at
            "#,
            payload.name,
            payload.username,
            hashed_password,
            payload.age,
            payload.gender as _
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    pub async fn update_user(
        db: &Db,
        id: i32,
        payload: UpdateUser,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                name = COALESCE($1, name),
                username = COALESCE($2, username),
                age = COALESCE($3, age),
                gender = COALESCE($4, gender)
            WHERE id = $5
            RETURNING id, name, username, password, age, gender as "gender: _", created_at
            "#,
            payload.name,
            payload.username,
            payload.age,
            payload.gender as _,
            id
        )
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    pub async fn delete_user(db: &Db, id: i32) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(db)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".into()));
        }

        Ok(())
    }
}
