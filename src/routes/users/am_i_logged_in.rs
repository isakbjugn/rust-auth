use axum::extract::State;
use axum::http;
use axum::response::IntoResponse;
use sqlx::PgPool;
use tower_cookies::Cookies;
use crate::db::{get_one_active_by_id};
use crate::types::tokens::AuthToken;
use crate::utils::{AppError, verify_auth_token};

#[tracing::instrument(name = "Getting info on a user if logged in", skip(state))]
pub async fn get(
    State(state): State<PgPool>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
    let auth_cookie = cookies.get("rust-auth").expect("No cookie found");
    let auth_token: AuthToken = verify_auth_token(auth_cookie.value().to_string()).await.map_err(|e| {
        tracing::event!(target: "backend", tracing::Level::ERROR, "Invalid token: {:#?}", e);
        AppError::Unauthorized
    })?;

    let user = get_one_active_by_id(&state, auth_token.user_id).await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Bruker ikke funnet i database: {:#?}", e);
            AppError::NotFound
        })?;

    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&user).unwrap())
        .unwrap();
    Ok(response)
}