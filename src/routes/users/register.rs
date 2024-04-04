use axum::{http, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::routes::users::User;
use crate::utils::auth::password::hash;
use crate::utils::error_handling::AppError;

#[derive(Deserialize, Serialize)]
pub struct NewUserRequest {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

pub async fn post(
    State(state): State<PgPool>,
    Json(new_user_request): Json<NewUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let hashed_password = hash(new_user_request.password.as_bytes()).await;
    let create_new_user = NewUser {
        email: new_user_request.email,
        password: hashed_password,
        first_name: new_user_request.first_name,
        last_name: new_user_request.last_name,
    };
    let user = create_user(&state, create_new_user).await?;

    let response = http::Response::builder()
        .status(http::StatusCode::CREATED)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&user).unwrap())
        .unwrap();
    Ok(response)
}

pub async fn create_user(
    db: &PgPool,
    new_user: NewUser,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password, first_name, last_name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, first_name, last_name, is_active",
        new_user.email,
        new_user.password,
        new_user.first_name,
        new_user.last_name
    ).fetch_one(db).await?;

    Ok(user)
}