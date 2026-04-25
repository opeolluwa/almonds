use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::todo::{TodoRepository, TodoRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError, types::todo::SyncTodoInput, utils::context::extract_db_conn,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncTodo;

#[CustomFields]
impl SyncTodo {
    async fn sync_todo(
        ctx: &Context<'_>,
        input: Vec<SyncTodoInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        let db = extract_db_conn(ctx)?;
        let repo = TodoRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::todo::Model> =
            input.into_iter().map(|item| item.into()).collect();

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(res)
    }
}
