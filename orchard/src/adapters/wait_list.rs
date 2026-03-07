use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate,)]
#[serde(rename_all = "camelCase")]
pub struct JoinWaitListRequest {
    #[validate(length(min = 1, message = "first name is required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "last name is required"))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize,)]
#[serde(rename_all = "camelCase")]
pub struct JoinWaitListResponse {
    pub identifier: String,
    pub email: String,
}
