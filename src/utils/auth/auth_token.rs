use pasetors::{public, Public};
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::{AsymmetricPublicKey, AsymmetricSecretKey};
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;

use crate::settings::get_setting;
use crate::types::tokens::AuthToken;
use crate::utils::AppError;

pub async fn issue_auth_token(user_id: uuid::Uuid) -> Result<String, AppError> {
    let current_date_time = chrono::Local::now();
    let dt = current_date_time + chrono::Duration::minutes(5);

    let mut claims = Claims::new().unwrap();
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims.add_additional("user_id", user_id.to_string()).unwrap();

    let secret = get_setting("ASYMMETRIC_SECRET_KEY");
    let hmac_secret = get_setting("HMAC_SECRET");
    let secret_key = AsymmetricSecretKey::<V4>::try_from(secret.as_str())?;

    Ok(public::sign(
        &secret_key,
        &claims,
        None,
        Some(hmac_secret.as_bytes()),
    )
        .unwrap()
    )
}

pub async fn verify_auth_token(token: String) -> Result<AuthToken, AppError> {
    let secret = get_setting("ASYMMETRIC_PUBLIC_KEY");
    let hmac_secret = get_setting("HMAC_SECRET");

    let public_key = AsymmetricPublicKey::<V4>::try_from(secret.as_str())?;

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Public, V4>::try_from(&token)?;
    let trusted_token = public::verify(
        &public_key,
        &untrusted_token,
        &validation_rules,
        None,
        Some(hmac_secret.as_bytes()),
    )?;
    let claims = trusted_token.payload_claims().unwrap();
    let user_id = serde_json::to_value(claims.get_claim("user_id").unwrap()).unwrap();

    match serde_json::from_value::<String>(user_id) {
        Ok(user_id_string) => match uuid::Uuid::parse_str(&user_id_string) {
            Ok(user_id) => Ok(AuthToken { user_id }),
            Err(e) => Err(AppError::UuidError(format!("{}", e))),
        },
        Err(e) => Err(AppError::UuidError(format!("{}", e))),
    }
}