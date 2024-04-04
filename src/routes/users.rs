use axum::extract::State;
use axum::http;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use crate::error_handling::AppError;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
}

pub async fn get(
    State(state): State<PgPool>
) -> Result<impl IntoResponse, AppError> {
    let users = get_all(&state).await;

    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&users).unwrap())
        .unwrap();
    Ok(response)
}

pub async fn get_all(db: &PgPool) -> Vec<User> {
    sqlx::query_as!(
        User,
        "SELECT id, email, first_name, last_name, is_active
        FROM users"
    ).fetch_all(db).await.expect("Klarte ikke å hente brukere")
}