use almond_kernel::adapters::meta::RequestMeta;
use almond_kernel::enums::ItemType as RecycleBinItemType;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecycleBinEntry {
    pub item_id: Uuid,
    pub item_type: RecycleBinItemType,
    pub payload: String,
    pub workspace_identifier: Option<Uuid>,
    pub meta: Option<RequestMeta>,
}

impl From<CreateRecycleBinEntry> for almond_kernel::adapters::recycle_bin::CreateRecycleBinEntry {
    fn from(e: CreateRecycleBinEntry) -> Self {
        Self {
            item_id: e.item_id,
            item_type: e.item_type,
            payload: e.payload,
            workspace_identifier: e.workspace_identifier,
        }
    }
}
