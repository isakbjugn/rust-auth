use pasetors::{local, Local};
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;

use crate::settings::get_setting;
use crate::types::ConfirmationToken;
use crate::utils::AppError;

pub async fn issue_confirmation_token(user_id: uuid::Uuid) -> Result<String, AppError> {
    let current_date_time = chrono::Local::now();
    let dt = current_date_time + chrono::Duration::hours(1);

    let mut claims = Claims::new().unwrap();
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims.add_additional("user_id", user_id.to_string()).unwrap();

    let secret_key = get_setting("SECRET_KEY");
    let hmac_secret = get_setting("HMAC_SECRET");

    let sk = SymmetricKey::<V4>::try_from(secret_key.as_str())?;
    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(hmac_secret.as_bytes()),
    )
        .unwrap()
    )
}

pub async fn _verify_confirmation_token(token: String) -> Result<ConfirmationToken, String> {
    let secret_key = get_setting("SECRET_KEY");
    let hmac_secret = get_setting("HMAC_SECRET");

    let sk = SymmetricKey::<V4>::from(secret_key.as_bytes()).unwrap();

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)
        .map_err(|e| format!("TokenValidation: {}", e))?;
    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(hmac_secret.as_bytes()),
    )
        .map_err(|e| format!("Pasetor: {}", e))?;
    let claims = trusted_token.payload_claims().unwrap();
    let user_id = serde_json::to_value(claims.get_claim("user_id").unwrap()).unwrap();

    match serde_json::from_value::<String>(user_id) {
        Ok(user_id_string) => match uuid::Uuid::parse_str(&user_id_string) {
            Ok(user_id) => Ok(ConfirmationToken { user_id }),
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => Err(format!("{}", e)),
    }
}