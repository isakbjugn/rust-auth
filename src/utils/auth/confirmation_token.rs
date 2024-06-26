use pasetors::{local, Local};
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;

use crate::settings::settings;
use crate::types::ConfirmationToken;
use crate::types::tokens::TokenPurpose;
use crate::utils::AppError;

pub async fn issue_confirmation_token(user_id: uuid::Uuid, token_purpose: TokenPurpose) -> Result<String, AppError> {
    let current_date_time = chrono::Local::now();
    let dt = current_date_time + chrono::Duration::hours(1);

    let mut claims = Claims::new().unwrap();
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims.add_additional("user_id", user_id.to_string()).unwrap();
    claims.add_additional("purpose", token_purpose.to_string()).unwrap();
    
    let symmetric_key = SymmetricKey::<V4>::try_from(settings().paseto.symmetric_key.as_str())?;
    
    Ok(local::encrypt(
        &symmetric_key,
        &claims,
        None,
        Some(settings().paseto.hmac_secret.as_bytes()),
    )
        .unwrap()
    )
}

pub async fn verify_confirmation_token(token: String, token_purpose: TokenPurpose) -> Result<ConfirmationToken, AppError> {
    let symmetric_key = SymmetricKey::<V4>::try_from(settings().paseto.symmetric_key.as_str())?;

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)?;
    let trusted_token = local::decrypt(
        &symmetric_key,
        &untrusted_token,
        &validation_rules,
        None,
        Some(settings().paseto.hmac_secret.as_bytes()),
    )?;
    
    let claims = trusted_token.payload_claims().unwrap();
    token_purpose.verify_claim(&claims)?;

    let user_id = serde_json::to_value(claims.get_claim("user_id").unwrap()).unwrap();

    match serde_json::from_value::<String>(user_id) {
        Ok(user_id_string) => match uuid::Uuid::parse_str(&user_id_string) {
            Ok(user_id) => Ok(ConfirmationToken { user_id }),
            Err(e) => Err(AppError::UuidError(format!("{}", e))),
        },
        Err(e) => Err(AppError::UuidError(format!("{}", e))),
    }
}