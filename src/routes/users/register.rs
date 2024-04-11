use axum::{http, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::create_user::{create_user, NewUser};
use crate::utils::AppError;
use crate::utils::hash;

#[derive(Deserialize)]
pub struct NewUserRequest {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[tracing::instrument(name = "Registering a new user", skip(state, new_user_request))]
pub async fn post(
    State(state): State<PgPool>,
    Json(new_user_request): Json<NewUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let hashed_password = hash(new_user_request.password.as_bytes()).await;
    let create_new_user = NewUser {
        email: new_user_request.email,
        password: hashed_password,
        first_name: new_user_request.first_name,
        last_name: new_user_request.last_name,
    };
    let user = create_user(&state, create_new_user).await?;
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
        .body(String::from("Kontoen din er opprettet, og du kan følge aktiveringslenken vi sendte på e-post for å aktivere den."))
        .unwrap();
    Ok(response)
}