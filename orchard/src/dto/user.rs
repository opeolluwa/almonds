use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct UserEntity {
    pub email: String,
    #[ts(type = "string")]
    pub identifier: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub is_active: bool,
    pub profile_picture: Option<String>,
    pub username: Option<String>,
}
