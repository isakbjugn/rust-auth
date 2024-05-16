use axum::{http, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};

use crate::types::login::LoginRequest;
use crate::utils::AppError;
use crate::utils::auth::login::login;

#[tracing::instrument(name = "Logging in a user", skip(state, cookies, login_request), fields(email = %login_request.email))]
pub async fn post(
    State(state): State<PgPool>,
    cookies: Cookies,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let auth_token = login(state, login_request).await?;

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