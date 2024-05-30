use axum::Json;
use axum::extract::State;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::create_user::{create_user, NewUser};
use crate::utils::api_response::ApiResponse;
use crate::utils::AppError;
use crate::utils::hash;

#[derive(Deserialize)]
pub struct NewUserRequest {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[tracing::instrument(name = "Registering a new user", skip(state, new_user_request))]
pub async fn post(
    State(state): State<PgPool>,
    Json(new_user_request): Json<NewUserRequest>,
) -> Result<ApiResponse<()>, AppError> {
    let hashed_password = hash(new_user_request.password.as_bytes()).await;
    let create_new_user = NewUser {
        email: new_user_request.email,
        password: hashed_password,
        first_name: new_user_request.first_name,
        last_name: new_user_request.last_name,
    };
    
    match create_user(&state, create_new_user).await {
        Ok(user) => {
            let user_id = uuid::Uuid::parse_str(&user.id).unwrap();

            crate::utils::send_multipart_email(
                "RustAuth - På tide å aktivere brukeren din".to_string(),
                user_id,
                user.email,
                user.first_name,
                user.last_name,
                "verification_email.html",
            )
                .await
                .unwrap();
            
            Ok(ApiResponse::OK)
        },
        Err(AppError::Conflict(user)) => {
            let user_id = uuid::Uuid::parse_str(&user.id).unwrap();
            
            match user.is_active {
                true => {
                    crate::utils::send_multipart_email(
                        "RustAuth - Forsøkte du å bytte passord?".to_string(),
                        user_id,
                        user.email,
                        user.first_name,
                        user.last_name,
                        "registration_attempt.html",
                    )
                },
                false => {
                    crate::utils::send_multipart_email(
                        "RustAuth - Husk å aktivere brukeren din".to_string(),
                        user_id,
                        user.email,
                        user.first_name,
                        user.last_name,
                        "verification_email.html",
                    )
                }
            }.await.unwrap();
            
            Ok(ApiResponse::OK)
        },
        Err(e) => {
            Err(e)
        }
    }
}