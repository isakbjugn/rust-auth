use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserWithPasswordHash {
    pub id: String,
    pub password: String,
}