use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::notes::{NotesRepository, NotesRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError, types::note::SyncNoteInput, utils::context::extract_db_conn,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncNote;

#[CustomFields]
impl SyncNote {
    async fn sync_note(
        ctx: &Context<'_>,
        input: Vec<SyncNoteInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        let db = extract_db_conn(ctx)?;
        let repo = NotesRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::notes::Model> =
            input.into_iter().map(|item| item.into()).collect();

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(res)
    }
}
