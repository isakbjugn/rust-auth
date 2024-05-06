use axum::http;
use axum::response::IntoResponse;
use crate::utils::AppError;

pub mod users;

#[tracing::instrument(name = "Index route")]
pub async fn get() -> Result<impl IntoResponse, AppError> {
    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(String::from("Heisann, hoppsann!"))
        .unwrap();
    Ok(response)
}
