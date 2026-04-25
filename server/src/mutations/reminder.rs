use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::reminder::{ReminderRepository, ReminderRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError, types::reminder::SyncReminderInput,
    utils::context::extract_db_conn,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncReminder;

#[CustomFields]
impl SyncReminder {
    async fn sync_reminder(
        ctx: &Context<'_>,
        input: Vec<SyncReminderInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        let db = extract_db_conn(ctx)?;
        let repo = ReminderRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::reminder::Model> =
            input.into_iter().map(|item| item.into()).collect();

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(res)
    }
}
