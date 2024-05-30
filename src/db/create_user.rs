use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::types::users::User;
use crate::utils::AppError;

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[tracing::instrument(name = "Creating a new user in db", skip(db, new_user), fields(email = %new_user.email))]
pub async fn create_user(
    db: &PgPool,
    new_user: NewUser,
) -> Result<User, AppError> {
    match sqlx::query_as!(
        User,
        "INSERT INTO users (email, password, first_name, last_name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, first_name, last_name, is_active, is_admin",
        new_user.email,
        new_user.password,
        new_user.first_name,
        new_user.last_name
    ).fetch_one(db).await {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(db_error)) if db_error.is_unique_violation() => {
            let conflicting_user = get_one_by_email(db, new_user.email.clone()).await.map_err(|e| {
                tracing::error!("Failed to get conflicting user: {:?}", e);
                AppError::SQLError(e)
            })?;
            Err(AppError::Conflict(conflicting_user))
        },
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);
            Err(AppError::SQLError(e))
        }
    }
}

#[tracing::instrument(name = "Getting active user from db by email", skip(db))]
async fn get_one_by_email(db: &PgPool, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active, is_admin
        FROM users
        WHERE email = $1",
        email
    ).fetch_one(db).await
}