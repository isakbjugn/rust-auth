use lettre::AsyncTransport;
use minijinja::path_loader;
use crate::settings::{Environment, get_environment, get_setting, get_settings};

pub async fn send_email(
    sender_email: Option<String>,
    recipient_email: String,
    recipient_first_name: String,
    recipient_last_name: String,
    subject: impl Into<String>,
    html_content: impl Into<String>,
    text_content: impl Into<String>,
) -> Result<(), String> {
    let settings = get_settings().expect("Klarte ikke å hente innstillinger");
    let email = lettre::Message::builder()
        .from(
            format!(
                "{} <{}>",
                settings.email.app_user_display_name,
                if sender_email.is_some() {
                    sender_email.unwrap()
                } else {
                    settings.email.app_user.clone()
                }
            )
                .parse()
                .unwrap(),
        )
        .to(format!(
            "{} <{}>",
            [recipient_first_name, recipient_last_name].join(" "),
            recipient_email
        )
            .parse()
            .unwrap())
        .subject(subject)
        .multipart(
            lettre::message::MultiPart::alternative()
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_PLAIN)
                        .body(text_content.into()),
                )
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_HTML)
                        .body(html_content.into()),
                ),
        )
        .unwrap();

    let creds = lettre::transport::smtp::authentication::Credentials::new(
        settings.email.app_user,
        settings.email.app_password,
    );

    // Open a remote connection to gmail
    let mailer: lettre::AsyncSmtpTransport<lettre::Tokio1Executor> =
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&settings.email.host)
            .unwrap()
            .credentials(creds)
            .build();

    // Send the email
    match mailer.send(email).await {
        Ok(_) => {
            tracing::event!(target: "backend", tracing::Level::INFO, "E-post med aktiveringslenke sendt!");
            Ok(())
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Kunne ikke sende e-post: {:#?}", e);
            Err(format!("Kunne ikke sende e-post: {:#?}", e))
        }
    }
}

pub async fn send_multipart_email(
    subject: String,
    user_id: uuid::Uuid,
    recipient_email: String,
    recipient_first_name: String,
    recipient_last_name: String,
    template_name: &str,
) -> Result<(), String> {
    let application_base_url = get_setting("APPLICATION_BASE_URL");
    let application_port = get_setting("APPLICATION_PORT");
    let token_expiration: i64 = get_setting("TOKEN_EXPIRATION_MINUTES").parse().unwrap();

    let title = subject.clone();

    let issued_token = match crate::utils::issue_confirmation_token(user_id).await
    {
        Ok(t) => t,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{:#?}", e);
            return Err(format!("{:#?}", e));
        }
    };
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
                "{}/users/register/confirm/?token={}",
                web_address, issued_token,
            )
        }
    };
    let current_date_time = chrono::Local::now();
    let dt = current_date_time + chrono::Duration::minutes(token_expiration);

    let mut template_env = minijinja::Environment::new();
    template_env.set_loader(path_loader("templates"));

    let template = template_env.get_template(template_name).unwrap();
    let ctx = minijinja::context! {
        title => &title,
        confirmation_link => &confirmation_link,
        expiration_time => &token_expiration,
        exact_time => &dt.format("%A %B %d, %Y at %r").to_string()
    };
    let html_text = template.render(ctx).unwrap();

    let text = format!(
        r#"
        Tap the link below to confirm your email address.
        {}
        "#,
        confirmation_link
    );
    tokio::spawn(send_email(
        None,
        recipient_email,
        recipient_first_name,
        recipient_last_name,
        subject,
        html_text,
        text,
    ));
    Ok(())
}