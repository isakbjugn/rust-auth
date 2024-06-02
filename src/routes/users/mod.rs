use axum::Json;
use axum::response::IntoResponse;

use crate::extractors::auth_session::AuthSession;
use crate::utils::AppError;

pub mod all;
pub mod register;

#[tracing::instrument(name = "Getting info on a user if logged in", skip(user))]
pub async fn get(
    AuthSession(user): AuthSession,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(user))
}