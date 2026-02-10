use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserDto {
    pub identifier: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
