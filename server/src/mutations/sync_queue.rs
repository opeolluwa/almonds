use almond_kernel::sync_engine::{DataQueue, SyncEngine, SyncEngineTrait};
use axum::http::HeaderMap;
use rayon::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};

use crate::{
    config::AppConfig,
    entities::{self},
    errors::app_error::AppError,
};

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

        // convert to entities
        let resolved_entities = input
            .par_iter()
            .map(|item| resolve_to_entity(item))
            .collect::<Vec<_>>();

        Ok(true)
    }
}

pub enum EntityWrapper {
    Bookmark(entities::bookmark::Entity),
    Notes(entities::notes::Entity),
    NoteCategories(entities::note_categories::Entity),
    Snippets(entities::snippets::Entity),
    Todo(entities::todo::Entity),
    Reminder(entities::reminder::Entity),
    Workspaces(entities::workspaces::Entity),
    RecycleBin(entities::recycle_bin::Entity),
    SyncQueue(entities::sync_queue::Entity),
    UserPreference(entities::user_preference::Entity),
    NoOp,
}

fn resolve_to_entity(item: &almond_kernel::entities::sync_queue::Model) -> EntityWrapper {
    match item.table_name.as_str() {
        "bookmark" => EntityWrapper::Bookmark(entities::bookmark::Entity),
        "notes" => EntityWrapper::Notes(entities::notes::Entity),
        "note_categories" => EntityWrapper::NoteCategories(entities::note_categories::Entity),
        "snippets" => EntityWrapper::Snippets(entities::snippets::Entity),
        "todo" => EntityWrapper::Todo(entities::todo::Entity),
        "reminder" => EntityWrapper::Reminder(entities::reminder::Entity),
        "workspaces" => EntityWrapper::Workspaces(entities::workspaces::Entity),
        "recycle_bin" => EntityWrapper::RecycleBin(entities::recycle_bin::Entity),
        "sync_queue" => EntityWrapper::SyncQueue(entities::sync_queue::Entity),
        "user_preference" => EntityWrapper::UserPreference(entities::user_preference::Entity),

        _ => EntityWrapper::NoOp,
    }
}

// see which is newer between upstream and downstream and return the delta for syncing

fn resolve_delta<T>(upstream: T, downstream: T) -> Option<T>
where
    T: EntityTrait + ColumnTrait,
{
    todo!()
}
