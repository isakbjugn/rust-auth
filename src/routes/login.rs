use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::db::get_one_active_by_email_with_password_hash;
use crate::utils::{AppError, issue_auth_token, verify_password};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[tracing::instrument(name = "Logging in a user", skip(state, login_request), fields(email = %login_request.email))]
pub async fn post(
    State(state): State<PgPool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_one_active_by_email_with_password_hash(&state, login_request.email.clone()).await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Bruker ikke funnet i database: {:#?}", e);
            AppError::Unauthorized
        })?;

    verify_password(user.password.as_str(), login_request.password.as_bytes()).await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Feil passord: {:#?}", e);
            AppError::Unauthorized
        })?;

    let user_id = uuid::Uuid::parse_str(&user.id).unwrap();
    let auth_token = issue_auth_token(user_id).await?;
    
    let login_response = LoginResponse { token: auth_token };
    Ok(Json(login_response))
}