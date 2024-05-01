use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};

use crate::db::get_one_active_by_id;
use crate::types::tokens::AuthToken;
use crate::types::users::User;
use crate::utils::{AppError, verify_auth_token};

pub struct AuthSession(pub User);

// Based on https://docs.rs/axum/latest/axum/extract/index.html#implementing-fromrequestparts
#[async_trait]
impl<S> FromRequestParts<S> for AuthSession
    where
        PgPool: FromRef<S>,
        S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                tracing::event!(target: "backend", tracing::Level::ERROR, "Missing token: {:#?}", e);
                AppError::Unauthorized
            })?;

        let auth_cookie = match cookies.get("rust-auth") {
            Some(cookie) => cookie,
            None => {
                tracing::event!(target: "backend", tracing::Level::ERROR, "No cookie found");
                return Err(AppError::Unauthorized);
            }
        };
        let auth_token: AuthToken = verify_auth_token(auth_cookie.value().to_string()).await
            .map_err(|e| {
                tracing::event!(target: "backend", tracing::Level::ERROR, "Invalid token: {:#?}", e);
                AppError::Unauthorized
            })?;

        match get_one_active_by_id(&pool, auth_token.user_id).await {
            Ok(user) => Ok(Self(user)),
            Err(e) => {
                tracing::event!(target: "backend", tracing::Level::ERROR, "Bruker ikke funnet i database: {:#?}", e);
                cookies.remove(Cookie::new("rust-auth", ""));
                Err(AppError::NotFound)
            }
        }
    }
}