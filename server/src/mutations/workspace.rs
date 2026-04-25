use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::workspace::{WorkspaceRepository, WorkspaceRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError, types::workspace::SyncWorkspaceInput,
    utils::context::extract_db_conn,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncWorkspace;

#[CustomFields]
impl SyncWorkspace {
    async fn sync_workspace(
        ctx: &Context<'_>,
        input: Vec<SyncWorkspaceInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        let db = extract_db_conn(ctx)?;
        let repo = WorkspaceRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::workspaces::Model> =
            input.into_iter().map(|item| item.into()).collect();

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(res)
    }
}
