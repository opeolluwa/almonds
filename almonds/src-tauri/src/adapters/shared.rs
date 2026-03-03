use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Sanitizer, Validate)]
#[serde(rename_all = "camelCase")]

pub struct RequestMeta {
    #[sanitizer(trim)]
    #[validate(length(min = 1))]
    pub workspace_identifier: String,
    #[sanitizer(trim)]
    #[validate(length(min = 1))]
    pub user_identifier: String,
}
