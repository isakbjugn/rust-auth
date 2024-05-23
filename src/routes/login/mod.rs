use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use sqlx::PgPool;
use crate::types::login::LoginRequest;
use crate::utils::AppError;
use crate::utils::auth::login::login;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[tracing::instrument(name = "Logging in a user", skip(state, login_request), fields(email = %login_request.email))]
pub async fn post(
    State(state): State<PgPool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let auth_token = login(state, login_request).await?;
    let login_response = LoginResponse { token: auth_token };
    Ok(Json(login_response))
}