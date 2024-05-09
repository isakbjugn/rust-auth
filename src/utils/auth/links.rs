use crate::settings::settings;
use crate::utils::AppError;

pub async fn create_confirmation_link(user_id: uuid::Uuid, template_name: String) -> Result<String, AppError> {
    let issued_token = crate::utils::issue_confirmation_token(user_id).await?;
    let web_address = settings().base_url();

    let confirmation_link = {
        if template_name == "password_reset_email.html" {
            format!(
                "{}/users/password/confirm/change_password?token={}",
                web_address, issued_token,
            )
        } else {
            format!(
                "{}/users/register/confirm?token={}",
                web_address, issued_token,
            )
        }
    };
    Ok(confirmation_link)
}