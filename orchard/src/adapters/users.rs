use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;

use crate::entities::users;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    #[ts(type = "string")]
    pub identifier: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
    pub username: Option<String>,
    pub country_identifier: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Validate, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PartialUserProfile {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

impl From<users::Model> for UserProfile {
    fn from(user: users::Model) -> Self {
        UserProfile {
            identifier: user.identifier,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            profile_picture: user.profile_picture,
            username: user.username,
            country_identifier: user.country_identifier,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct BackupEmail {
    pub email: String,
}
