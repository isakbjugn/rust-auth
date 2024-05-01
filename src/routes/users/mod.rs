use axum::http;
use axum::response::IntoResponse;

use crate::extractors::auth_session::AuthSession;
use crate::utils::AppError;

pub mod all;
pub mod confirm_registration;
pub mod generate_new_token;
pub mod login;
pub mod register;

#[tracing::instrument(name = "Getting info on a user if logged in", skip(user))]
pub async fn get(
    AuthSession(user): AuthSession,
) -> Result<impl IntoResponse, AppError> {
    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&user).unwrap())
        .unwrap();
    Ok(response)
}