use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePlaylistRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]

pub struct UpdatePlaylistRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
