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
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password, first_name, last_name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, first_name, last_name, is_active, is_admin",
        new_user.email,
        new_user.password,
        new_user.first_name,
        new_user.last_name
    ).fetch_one(db).await.map_err(|e| {
        match e {
            sqlx::Error::Database(db_error) if db_error.is_unique_violation() => {
                AppError::Conflict(format!("User with email {} already exists", new_user.email))
            }
            _ => {
                tracing::error!("Failed to create user: {:?}", e);
                AppError::SQLError(e)
            }
        }
    })?;

    Ok(user)
}