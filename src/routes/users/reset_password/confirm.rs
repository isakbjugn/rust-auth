use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::change_password::change_password;
use crate::db::get_one_active_by_id;
use crate::types::tokens::TokenPurpose;
use crate::utils::{AppError, verify_confirmation_token};

#[derive(Deserialize)]
pub struct NewPassordRequest {
    token: String,
    new_password: String,
}

#[tracing::instrument(name = "Activating user", skip_all)]
pub async fn post(
    State(state): State<PgPool>,
    Json(new_password_request): Json<NewPassordRequest>

) -> Result<impl IntoResponse, AppError> {
    let confirmation_token = verify_confirmation_token(new_password_request.token.clone(), TokenPurpose::ResetPassword).await?;
    match get_one_active_by_id(&state, confirmation_token.user_id).await {
        Ok(user) => {
            let user_id = uuid::Uuid::parse_str(&user.id)
                .map_err(|e| AppError::ParseError(format!("{}", e)))?;
            let hashed_password = crate::utils::hash(new_password_request.new_password.as_bytes()).await;
            change_password(&state, user_id, hashed_password).await?;

            Ok(StatusCode::OK)
        },
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Forsøkte å endre passord for uaktivert bruker: {:#?}", e);
            Err(AppError::NotFound)
        },
    }
}