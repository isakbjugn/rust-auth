use pasetors::claims::Claims;
use serde::{Deserialize, Serialize};
use crate::utils::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfirmationToken {
    pub user_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthToken {
    pub user_id: uuid::Uuid,
}


#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum TokenPurpose {
    Activate,
    ResetPassword,
}

impl std::fmt::Display for TokenPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenPurpose::Activate => write!(f, "activate"),
            TokenPurpose::ResetPassword => write!(f, "reset_password"),
        }
    }
}

impl TokenPurpose {
    pub fn verify_claim(&self, claims: &Claims) -> Result<(), AppError> {
        let purpose_value = serde_json::to_value(claims.get_claim("purpose").unwrap()).unwrap();
        match serde_json::from_value::<String>(purpose_value) {
            Ok(purpose) if purpose == self.to_string() => Ok(()),
            Ok(_) => Err(AppError::Forbidden("Incorrect token purpose".to_string())),
            Err(e) => Err(AppError::ParseError(format!("{}", e)))
        }
    }
}