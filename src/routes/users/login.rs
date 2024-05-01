use axum::extract::State;
use axum::{http, Json};
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};
use crate::db::get_one_active_by_email_with_password_hash;
use crate::utils::{AppError, issue_auth_token, verify_password};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[tracing::instrument(name = "Logging in a user", skip(state, cookies, login_request), fields(email = %login_request.email))]
pub async fn post(
    State(state): State<PgPool>,
    cookies: Cookies,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
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
    let auth_token = issue_auth_token(user_id).await?;

    let auth_cookie_builder = Cookie::build(("rust-auth", auth_token))
        .domain("localhost")
        .path("/")
        .secure(true)
        .http_only(true);

    cookies.add(Cookie::from(auth_cookie_builder));

    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(String::from("Du er nå logget inn!"))
        .unwrap();
    Ok(response)
}