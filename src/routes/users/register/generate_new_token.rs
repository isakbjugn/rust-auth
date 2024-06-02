use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::get_one_by_email;
use crate::utils::AppError;

#[derive(Deserialize)]
pub struct GenerateTokenRequest {
    email: String
}

#[tracing::instrument(name = "Generating new token", skip(state, generate_token_request))]
pub async fn post(
    State(state): State<PgPool>,
    Json(generate_token_request): Json<GenerateTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    match get_one_by_email(&state, generate_token_request.email.clone()).await {
        Ok(inactive_user) if !inactive_user.is_active => {
            let user_id = uuid::Uuid::parse_str(&inactive_user.id).unwrap();
            crate::utils::send_multipart_email(
                "RustAuth - På tide å aktivere brukeren din".to_string(),
                user_id,
                inactive_user.email,
                inactive_user.first_name,
                inactive_user.last_name,
                "verification_email.html",
            )
                .await
                .expect("Klarte ikke å sende e-post!");
            
            Ok(StatusCode::OK)
        },
        Ok(active_user) if active_user.is_active => {
            let user_id = uuid::Uuid::parse_str(&active_user.id).unwrap();
            crate::utils::send_multipart_email(
                "RustAuth - Forsøkte du å bytte passord?".to_string(),
                user_id,
                active_user.email,
                active_user.first_name,
                active_user.last_name,
                "registration_attempt.html",
            )
                .await
                .expect("Klarte ikke å sende e-post!");

            Ok(StatusCode::OK)
        },
        Ok(_) => unreachable!(),
        Err(AppError::NotFound) => {
            crate::utils::emails::send_email_about_registration_attempt(
                "RustAuth - Registreringsforsøk".to_string(),
                generate_token_request.email.clone(),
                "Ukjent".to_string(),
                "mottaker".to_string(),
                "registration_attempt.html",
            )
                .await
                .expect("Klarte ikke å sende e-post!");
            
            Ok(StatusCode::OK)
        },
        Err(e) => Err(e),
    }
}