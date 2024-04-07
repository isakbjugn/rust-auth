pub mod create_user;
pub mod activate_user;

use sqlx::PgPool;
use crate::types::users::User;

pub async fn get_all(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active
        FROM users"
    ).fetch_all(db).await
}

pub async fn get_one_inactive_by_id(db: &PgPool, id: uuid::Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active
        FROM users
        WHERE id = $1 AND is_active = false",
        id
    ).fetch_one(db).await
}

pub async fn get_one_inactive_by_email(db: &PgPool, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active
        FROM users
        WHERE email = $1 AND is_active = false",
        email
    ).fetch_one(db).await
}