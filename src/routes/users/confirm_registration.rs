use axum::extract::State;
use axum::{http, Json};
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;
use crate::db::activate_user::activate_user;
use crate::db::get_one_inactive_by_id;
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
    let confirmation_token = verify_confirmation_token(parameters.token.clone()).await?;
    let user = get_one_inactive_by_id(&state, confirmation_token.user_id).await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Brukeren er allerede aktivert: {:#?}", e);
            AppError::AlreadyActivated
        })?;
    let user_id = uuid::Uuid::parse_str(&user.id).unwrap();
    activate_user(&state, user_id).await?;

    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(String::from("Kontoen din er aktivert."))
        .unwrap();
    Ok(response)
}