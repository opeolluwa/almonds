use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::reminder::{ReminderRepository, ReminderRepositoryExt},
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{errors::app_error::AppError, utils::context::extract_db_conn};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncReminder;

#[CustomFields]
impl SyncReminder {
    async fn sync_reminder(
        ctx: &Context<'_>,
        input: Vec<entities::reminder::Model>,
    ) -> async_graphql::Result<bool> {
        let db = extract_db_conn(ctx)?;
        let repo = ReminderRepository::new(Arc::new(db.clone()));
        repo.upsert_many(input)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        Ok(true)
    }
}
