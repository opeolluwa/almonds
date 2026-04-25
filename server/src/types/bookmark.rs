use almond_kernel::entities;
use almond_kernel::entities::sea_orm_active_enums::Tag;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SyncBookmarkInput {
    pub identifier: Uuid,
    pub title: String,
    pub url: String,
    pub tag: Tag,
    pub created_at: String,
    pub updated_at: String,
    pub workspace_identifier: Option<Uuid>,
}

impl From<SyncBookmarkInput> for entities::bookmark::Model {
    fn from(val: SyncBookmarkInput) -> Self {
        entities::bookmark::Model {
            identifier: val.identifier,
            title: val.title,
            url: val.url,
            tag: val.tag,
            created_at: val.created_at.parse().unwrap(),
            updated_at: val.updated_at.parse().unwrap(),
            workspace_identifier: val.workspace_identifier,
        }
    }
}
