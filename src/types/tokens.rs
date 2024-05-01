use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfirmationToken {
    pub user_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthToken {
    pub user_id: uuid::Uuid,
    pub is_admin: bool,
}