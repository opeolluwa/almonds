use almond_kernel::sync_engine::{DataQueue, SyncEngine, SyncEngineTrait};
use axum::http::HeaderMap;
use sea_orm::DatabaseConnection;
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};

use crate::{cli::app, config::AppConfig, entities::sync_queue, errors::app_error::AppError};

pub struct SyncQueue;

impl SyncQueue {
    async fn sync_engine(
        db: DatabaseConnection,
        api_url: &str,
        api_key: &str,
        resource_path: &str,
    ) -> Result<SyncEngine, AppError> {
        SyncEngine::new(db, api_url, api_key, resource_path)
            .await
            .map_err(|err| AppError::InternalError(err.to_string()))
    }
}
#[CustomFields]
impl SyncQueue {
    async fn sync_queue(ctx: &Context<'_>, input: DataQueue) -> async_graphql::Result<bool> {
        let db_conn = ctx
            .data::<DatabaseConnection>()
            .map_err(|err| AppError::InternalError(err.message))?;

        let headers = ctx
            .data::<HeaderMap>()
            .map_err(|_| AppError::InternalError("Missing request headers".to_string()))?;

        let api_key = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::InternalError("Missing Authorization header".to_string()))?;

        let app_config = AppConfig::from_env()?;

        let sync_engine = Self::sync_engine(
            db_conn.to_owned(),
            &app_config.base_url,
            api_key,
            &app_config.graphql_endpoint,
        )
        .await?;

        let res = sync_engine.up_sync(input).await?;
        Ok(true)
    }
}
