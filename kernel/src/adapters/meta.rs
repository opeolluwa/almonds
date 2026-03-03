use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Sanitizer, Validate, Clone)]
#[serde(rename_all = "camelCase")]

pub struct RequestMeta {
    // #[sanitizer(trim)]
    // #[validate(length(min = 1))]
    pub workspace_identifier: Uuid,
    // #[sanitizer(trim)]
    // #[validate(length(min = 1))]
    // pub user_identifier: Uuid,
}
