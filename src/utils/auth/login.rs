use sqlx::PgPool;
use crate::db::get_one_active_by_email_with_password_hash;
use crate::types::login::LoginRequest;
use crate::utils::{AppError, issue_auth_token, verify_password};

pub async fn login(
    state: PgPool,
    login_request: LoginRequest,
) -> Result<String, AppError> {
    let user = get_one_active_by_email_with_password_hash(&state, login_request.email.clone()).await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Bruker ikke funnet i database: {:#?}", e);
            AppError::NotFound
        })?;

    verify_password(user.password.as_str(), login_request.password.as_bytes()).await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Feil passord: {:#?}", e);
            AppError::Unauthorized
        })?;

    let user_id = uuid::Uuid::parse_str(&user.id).unwrap();
    issue_auth_token(user_id).await
}