use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfirmationToken {
    pub user_id: uuid::Uuid,
}