use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserEntity {
    pub email: String,
    pub identifier: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub is_active: bool,
}
