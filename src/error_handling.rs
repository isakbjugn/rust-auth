use axum::{http, response::IntoResponse};
use tracing::error;

pub enum AppError {
    SQLError(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::SQLError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::SQLError(err) => {
                error!("SQL error: {:?}", err);
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        .into_response()
    }
}