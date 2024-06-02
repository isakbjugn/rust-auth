use crate::settings::settings;
use crate::types::tokens::TokenPurpose;
use crate::utils::AppError;

#[tracing::instrument(name = "Creating confirmation link", skip(user_id))]
pub async fn create_confirmation_link(user_id: uuid::Uuid, token_purpose: TokenPurpose) -> Result<String, AppError> {
    let issued_token = crate::utils::issue_confirmation_token(user_id, token_purpose).await?;
    let web_address = settings().frontend.url.clone();

    let confirmation_link = match token_purpose {
        TokenPurpose::Activate => {
            format!(
                "{}/register/confirm?token={}",
                web_address, issued_token,
            )
        },
        TokenPurpose::ResetPassword => {
            format!(
                "{}/users/password/change-password?token={}",
                web_address, issued_token,
            )
        },
    };
    Ok(confirmation_link)
}