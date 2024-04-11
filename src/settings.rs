use serde::Deserialize;

pub enum Environment {
    Development,
    Production,
}

pub fn get_setting(key: &str) -> String {
    dotenv::dotenv().ok();
    std::env::var(key).expect("Klarte ikke å lese {} fra miljøvariabler.")
}

pub fn get_environment() -> Environment {
    dotenv::dotenv().ok();
    match std::env::var("ENVIRONMENT").expect("Klarte ikke å lese ENVIRONMENT fra miljøvariabler.") {
        s if s == "development" => Environment::Development,
        s if s == "production" => Environment::Production,
        _ => panic!("Ugyldig ENVIRONMENT-verdi. Mulige verdier er 'development' og 'production'."),
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub email: EmailSettings,
}

#[derive(Deserialize, Clone)]
pub struct EmailSettings {
    pub host: String,
    pub app_user: String,
    pub app_password: String,
    pub app_user_display_name: String,
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Klarte ikke å finne gjeldende katalog");
    let settings_directory = base_path.join("settings");

    let settings = config::Config::builder()
        .add_source(config::File::from(settings_directory.join("base.toml")))
        .build()?;

    settings.try_deserialize::<Settings>()
}