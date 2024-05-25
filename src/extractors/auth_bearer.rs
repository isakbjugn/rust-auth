use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;

use crate::utils::AppError;

pub struct AuthBearer(pub String);

impl AuthBearer {
    pub fn from_header(contents: &str) -> Self {
        Self(contents.to_string())
    }
    pub fn token(&self) -> &str {
        &self.0
    }
}

// Based on https://docs.rs/axum/latest/axum/extract/index.html#implementing-fromrequestparts
#[async_trait]
impl<S> FromRequestParts<S> for AuthBearer
    where
        S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers
            .get(AUTHORIZATION)
            .ok_or(AppError::Unauthorized)?
            .to_str()
            .map_err(|e| AppError::ParseError(e.to_string()))?;

        let split = auth_header.split_once(' ');
        match split {
            // Found proper bearer
            Some((name, contents)) if name == "Bearer" => Ok(Self::from_header(contents)),
            // Found empty bearer; sometimes request libraries format them as this
            _ if auth_header == "Bearer" => Ok(Self::from_header("")),
            // Found nothing
            _ => {
                tracing::event!(target: "backend", tracing::Level::ERROR, "Ingen Authorization Bearer-header");
                Err(AppError::Unauthorized)
            }
        }
    }
}