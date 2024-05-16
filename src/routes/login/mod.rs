pub mod local;

use axum::extract::State;
use axum::{http, Json};
use axum::response::IntoResponse;
use sqlx::PgPool;
use crate::types::login::LoginRequest;
use crate::utils::AppError;
use crate::utils::auth::login::login;

#[tracing::instrument(name = "Logging in a user", skip(state, login_request), fields(email = %login_request.email))]
pub async fn post(
    State(state): State<PgPool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let auth_token = login(state, login_request).await?;

    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .header(http::header::AUTHORIZATION, format!("Bearer {}", auth_token))
        .body(String::from("Du er nå logget inn!"))
        .unwrap();
    Ok(response)
}