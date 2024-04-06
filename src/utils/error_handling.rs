use axum::{http, response::IntoResponse};
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    PasetoError(pasetors::errors::Error),
    SQLError(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::SQLError(error)
    }
}

impl From<pasetors::errors::Error> for AppError {
    fn from(error: pasetors::errors::Error) -> Self {
        Self::PasetoError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => http::StatusCode::NOT_FOUND,
            AppError::PasetoError(err) => {
                error!("Paseto error: {:?}", err);
                http::StatusCode::INTERNAL_SERVER_ERROR
            },
            AppError::SQLError(err) => {
                error!("SQL error: {:?}", err);
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        .into_response()
    }
}