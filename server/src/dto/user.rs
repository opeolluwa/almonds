use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    pub email: String,
    pub identifier: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub is_active: bool,
    pub profile_picture: Option<String>,
    pub username: Option<String>,
}
