use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, sync_queue::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncQueueEntry {
    pub table_name: String,
    pub record_identifier: String,
    pub operation: String,
    pub created_at: String,
}

impl Into<entities::sync_queue::ActiveModel> for SyncQueueEntry {
    fn into(self) -> entities::sync_queue::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            table_name: Set(self.table_name),
            record_identifier: Set(self.record_identifier),
            operation: Set(self.operation),
            created_at: Set(self.created_at),
        }
    }
}
