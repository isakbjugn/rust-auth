use axum::{http, Json};
use axum::response::{IntoResponse, Response};

pub enum ApiResponse<T> {
    JsonData(T),
    OK,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::JsonData(data) => Json(data).into_response(),
            Self::OK => http::StatusCode::OK.into_response(),
        }
    }
}