use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::db::get_all;
use crate::extractors::auth_session::AuthSession;
use crate::utils::AppError;

#[tracing::instrument(name = "Getting all users", skip(state))]
pub async fn get(
    State(state): State<PgPool>,
    AuthSession(user): AuthSession
) -> Result<impl IntoResponse, AppError> {
    let users = match user.is_admin {
        true => get_all(&state).await,
        false => return Err(AppError::Unauthorized),
    }?;

    Ok(Json(users))
}