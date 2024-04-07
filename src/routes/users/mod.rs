use axum::extract::State;
use axum::http;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::db::get_all;
use crate::utils::AppError;

pub mod register;
pub mod generate_new_token;
pub mod confirm_registration;

pub async fn get(
    State(state): State<PgPool>
) -> Result<impl IntoResponse, AppError> {
    let users = get_all(&state).await?;

    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&users).unwrap())
        .unwrap();
    Ok(response)
}