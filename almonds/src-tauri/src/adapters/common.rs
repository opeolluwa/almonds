use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Sanitizer)]
#[serde(rename_all = "camelCase")]

pub struct RequestMeta {
    #[sanitizer(trim)]
    pub workspace_identifier: String,
    #[sanitizer(trim)]
    pub user_identifier: String,
}
