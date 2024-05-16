use axum::response::{Html, IntoResponse};
use crate::utils::AppError;

pub mod login;
pub mod users;

#[tracing::instrument(name = "Index route")]
pub async fn get() -> Result<impl IntoResponse, AppError> {
    Ok(Html("<h1>Heisann, hoppsann!</h1>"))
}
