use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::activate_user::activate_user;
use crate::types::tokens::TokenPurpose;
use crate::utils::{AppError, verify_confirmation_token};

#[derive(Deserialize)]
pub struct Parameters {
    token: String,
}

#[tracing::instrument(name = "Activating user", skip(state, parameters))]
pub async fn post(
    State(state): State<PgPool>,
    Json(parameters): Json<Parameters>
) -> Result<impl IntoResponse, AppError> {
    let confirmation_token = verify_confirmation_token(parameters.token.clone(), TokenPurpose::Activate).await?;
    match activate_user(&state, confirmation_token.user_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Brukeren er allerede aktivert: {:#?}", e);
            Ok(StatusCode::OK)
        }
    }
}