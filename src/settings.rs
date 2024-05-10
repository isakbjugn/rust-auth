use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ApplicationSettings {
    pub protocol: String,
    pub host: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct EmailSettings {
    pub host: String,
    pub app_user: String,
    pub app_password: String,
    pub app_user_display_name: String,
}

#[derive(Debug, Deserialize)]
pub enum Environment {
    Development,
    Production,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PasetoSettings {
    pub symmetric_key: String,
    pub asymmetric_secret_key: String,
    pub asymmetric_public_key: String,
    pub hmac_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TenantSettings {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TokenSettings {
    pub expiration_minutes: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database_url: String,
    pub email: EmailSettings,
    pub environment: Environment,
    pub paseto: PasetoSettings,
    pub port: u16,
    pub tenant: TenantSettings,
    pub token: TokenSettings,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();
        // https://github.com/mehcode/config-rs/blob/master/examples/hierarchical-env/settings.rs
        let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());

        let settings = Config::builder()
            .add_source(File::with_name("settings/base"))
            .add_source(
                File::with_name(&format!("settings/{}", environment))
                    .required(false),
            )
            // Tilstrekkelig for DATABASE_URL -> database.url
            .add_source(config::Environment::default()
                .separator("__")
            )
            .set_override("environment", environment)?
            .build()?;

        settings.try_deserialize()
    }
    pub fn base_url(&self) -> String {
        let base_without_port = format!("{}://{}", self.application.protocol, self.application.host);
        match self.environment {
            Environment::Development => format!("{}:{}", base_without_port, self.port),
            Environment::Production => base_without_port,
        }
    }
}

pub fn settings() -> &'static Settings {
    static SETTINGS: std::sync::OnceLock<Settings> = std::sync::OnceLock::new();
    SETTINGS.get_or_init(|| Settings::new().expect("Klarte ikke å laste inn konfigurasjonen"))
}