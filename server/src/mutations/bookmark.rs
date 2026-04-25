use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::bookmarks::{BookmarkRepository, BookmarkRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError, types::bookmark::SyncBookmarkInput,
    utils::context::extract_db_conn,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncBookmark;

#[CustomFields]
impl SyncBookmark {
    async fn sync_bookmark(
        ctx: &Context<'_>,
        input: Vec<SyncBookmarkInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        let db = extract_db_conn(ctx)?;
        let repo = BookmarkRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::bookmark::Model> =
            input.into_iter().map(|item| item.into()).collect();

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(res)
    }
}
