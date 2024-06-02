pub mod create_user;
pub mod activate_user;

use sqlx::PgPool;
use crate::types::users::{User, UserWithPasswordHash};
use crate::utils::AppError;

#[tracing::instrument(name = "Getting all users from db", skip(db))]
pub async fn get_all(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active, is_admin
        FROM users"
    ).fetch_all(db).await
}

#[tracing::instrument(name = "Get one user from db", skip(db))]
pub async fn get_one_by_email(db: &PgPool, email: String) -> Result<User, AppError> {
    match sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active, is_admin
        FROM users
        WHERE email = $1",
        email
    ).fetch_one(db).await {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(db_error)) if db_error.message() == "RowNotFound" => {
            Err(AppError::NotFound)
        },
        Err(sqlx::Error::RowNotFound) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "DB-feil: Kunne ikke finne bruker med e-post: {:#?}", email);
            Err(AppError::NotFound)
        },
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Kunne ikke sende e-post: {:#?}", e);
            Err(AppError::SQLError(e))
        }
    }
}

#[tracing::instrument(name = "Getting active user from db by id", skip(db))]
pub async fn get_one_active_by_id(db: &PgPool, id: uuid::Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active, is_admin
        FROM users
        WHERE id = $1 AND is_active = true",
        id
    ).fetch_one(db).await
}

#[tracing::instrument(name = "Getting inactive user from db by id", skip(db))]
pub async fn get_one_inactive_by_id(db: &PgPool, id: uuid::Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active, is_admin
        FROM users
        WHERE id = $1 AND is_active = false",
        id
    ).fetch_one(db).await
}

#[tracing::instrument(name = "Getting active user from db by email", skip(db))]
pub async fn get_one_active_by_email_with_password_hash(db: &PgPool, email: String) -> Result<UserWithPasswordHash, sqlx::Error> {
    sqlx::query_as!(
        UserWithPasswordHash,
        "SELECT id, password
        FROM users
        WHERE email = $1 AND is_active = true",
        email
    ).fetch_one(db).await
}

#[tracing::instrument(name = "Getting inactive user from db by email", skip(db))]
pub async fn get_one_inactive_by_email(db: &PgPool, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active, is_admin
        FROM users
        WHERE email = $1 AND is_active = false",
        email
    ).fetch_one(db).await
}