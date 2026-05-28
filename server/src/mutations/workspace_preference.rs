use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::workspace_preferences::{
        WorkspacePreferenceRepository, WorkspacePreferenceRepositoryExt,
    },
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError, types::workspace_preferences::SyncWorkspacePreferenceInput,
    utils::context::extract_db_conn,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncUserPreference;

#[CustomFields]
impl SyncUserPreference {
    async fn sync_user_preference(
        ctx: &Context<'_>,
        input: Vec<SyncWorkspacePreferenceInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        let db = extract_db_conn(ctx)?;
        let repo = WorkspacePreferenceRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::workspace_preferences::Model> = input
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<_, _>>()?;

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(res)
    }
}
