use std::sync::Arc;

use almond_kernel::{
    entities,
    repositories::user_preference::{UserPreferenceRepository, UserPreferenceRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError, types::user_preference::SyncUserPreferenceInput,
    utils::context::extract_db_conn,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncUserPreference;

#[CustomFields]
impl SyncUserPreference {
    async fn sync_user_preference(
        ctx: &Context<'_>,
        input: Vec<SyncUserPreferenceInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        let db = extract_db_conn(ctx)?;
        let repo = UserPreferenceRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::user_preference::Model> =
            input.into_iter().map(|item| item.into()).collect();

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(res)
    }
}
