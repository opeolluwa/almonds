use almond_kernel::adapters::recycle_bin::RecycleBinItemType;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecycleBinEntry {
    pub item_id: Uuid,
    pub item_type: RecycleBinItemType,
    pub payload: String,
}

impl From<CreateRecycleBinEntry> for almond_kernel::adapters::recycle_bin::CreateRecycleBinEntry {
    fn from(e: CreateRecycleBinEntry) -> Self {
        Self {
            item_id: e.item_id,
            item_type: e.item_type,
            payload: e.payload,
        }
    }
}
