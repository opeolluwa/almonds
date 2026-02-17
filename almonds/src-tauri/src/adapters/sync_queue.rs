use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSyncQueueEntry {
    pub table_name: String,
    pub record_identifier: String,
    pub operation: String,
    pub created_at: String,
}

impl From<AddSyncQueueEntry> for almond_kernel::adapters::sync_queue::SyncQueueEntry {
    fn from(entry: AddSyncQueueEntry) -> Self {
        Self {
            table_name: entry.table_name,
            record_identifier: entry.record_identifier,
            operation: entry.operation,
            created_at: entry.created_at,
        }
    }
}
