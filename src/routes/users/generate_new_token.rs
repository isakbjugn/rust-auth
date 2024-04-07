use axum::{http, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;
use crate::db::get_one_inactive_by_email;

use crate::utils::AppError;

#[derive(Deserialize)]
pub struct GenerateTokenRequest {
    email: String
}

pub async fn post(
    State(state): State<PgPool>,
    Json(generate_token_request): Json<GenerateTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_one_inactive_by_email(&state, generate_token_request.email.clone()).await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Bruker ikke funnet i database: {:#?}", e);
            AppError::NotFound
        })?;
    let user_id = uuid::Uuid::parse_str(&user.id).unwrap();

    crate::utils::send_multipart_email(
        "RustAuth - På tide å aktivere brukeren din".to_string(),
        user_id,
        user.email,
        user.first_name,
        user.last_name,
        "verification_email.html",
    )
        .await
        .unwrap();

    let response = http::Response::builder()
        .status(http::StatusCode::CREATED)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(String::from("En ny aktiveringslenke er sendt til din e-epostadresse."))
        .unwrap();
    Ok(response)
}