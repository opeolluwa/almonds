use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::bookmarks::{BookmarkRepository, BookmarkRepositoryExt},
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{errors::app_error::AppError, utils::context::extract_db_conn};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncBookmark;

#[CustomFields]
impl SyncBookmark {
    async fn sync_bookmark(
        ctx: &Context<'_>,
        input: Vec<entities::bookmark::Model>,
    ) -> async_graphql::Result<bool> {
        let db = extract_db_conn(ctx)?;
        let repo = BookmarkRepository::new(Arc::new(db.clone()));
        repo.upsert_many(input)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        Ok(true)
    }
}
