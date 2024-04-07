use crate::settings::{Environment, get_environment, get_setting};
use crate::utils::AppError;

pub async fn create_confirmation_link(user_id: uuid::Uuid, template_name: String) -> Result<String, AppError> {
    let application_base_url = get_setting("APPLICATION_BASE_URL");
    let application_port = get_setting("APPLICATION_PORT");

    let issued_token = crate::utils::issue_confirmation_token(user_id).await?;
    let web_address = {
        match get_environment() {
            Environment::Development => format!(
                "{}:{}",
                application_base_url,
                application_port
            ),
            Environment::Production => application_base_url
        }
    };
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