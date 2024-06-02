use axum::{http::StatusCode, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::get_one_by_email;
use crate::types::tokens::TokenPurpose;
use crate::utils::AppError;

pub mod confirm;

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    email: String,
}

#[tracing::instrument(name = "Issuing token for password reset", skip(state, reset_password_request))]
pub async fn post(
    State(state): State<PgPool>,
    Json(reset_password_request): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    match get_one_by_email(&state, reset_password_request.email.clone()).await {
        Ok(inactive_user) if !inactive_user.is_active => {
            let user_id = uuid::Uuid::parse_str(&inactive_user.id).unwrap();
            crate::utils::send_multipart_email(
                "RustAuth - På tide å aktivere brukeren din".to_string(),
                user_id,
                inactive_user.email,
                inactive_user.first_name,
                inactive_user.last_name,
                TokenPurpose::Activate,
            )
                .await
                .expect("Klarte ikke å sende e-post!");

            Ok(StatusCode::OK)
        },
        Ok(active_user) if active_user.is_active => {
            let user_id = uuid::Uuid::parse_str(&active_user.id).unwrap();
            crate::utils::send_multipart_email(
                "RustAuth - Instruksjoner for å tilbakestille passord".to_string(),
                user_id,
                active_user.email,
                active_user.first_name,
                active_user.last_name,
                TokenPurpose::ResetPassword,
            )
                .await
                .expect("Klarte ikke å sende e-post!");

            Ok(StatusCode::OK)
        },
        Ok(_) => unreachable!(),
        Err(AppError::NotFound) => {
            crate::utils::emails::send_email_about_registration_attempt(
                "RustAuth - Tilbakestillingsforsøk".to_string(),
                reset_password_request.email.clone(),
                "Ukjent".to_string(),
                "mottaker".to_string(),
            )
                .await
                .expect("Klarte ikke å sende e-post!");

            Ok(StatusCode::OK)
        },
        Err(e) => Err(e),
    }
}