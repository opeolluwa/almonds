use almond_kernel::entities;
use almond_kernel::sync_engine::{DataQueue, SyncEngine, SyncEngineTrait};
use rayon::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, Statement};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};

use crate::{
    config::AppConfig, errors::app_error::AppError, utils::context::extract_request_context,
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
        let req_ctx = extract_request_context(ctx)?;
        let app_config = AppConfig::from_env()?;

        let filtered = filter_stale_items(req_ctx.db_conn, input).await?;

        if filtered.is_empty() {
            return Ok(true);
        }

        let sync_engine = Self::sync_engine(
            req_ctx.db_conn.to_owned(),
            &app_config.base_url,
            req_ctx.api_key,
            &app_config.graphql_endpoint,
        )
        .await?;

        sync_engine
            .sync(filtered)
            .await
            .map_err(|err| AppError::InternalError(err.to_string()))?;

        Ok(true)
    }
}

#[derive(Debug, FromQueryResult)]
struct RecordTimestamp {
    updated_at: String,
}

/// Drops any item whose local record was updated more recently than the incoming
/// change — keeping only items where the incoming change is the newer version.
async fn filter_stale_items(
    db: &DatabaseConnection,
    items: DataQueue,
) -> Result<DataQueue, AppError> {
    let backend = db.get_database_backend();
    let mut result = Vec::with_capacity(items.len());

    for item in items {
        let sql = format!(
            "SELECT updated_at FROM \"{}\" WHERE identifier = ?",
            item.table_name
        );
        let stmt =
            Statement::from_sql_and_values(backend, &sql, [item.record_identifier.clone().into()]);

        let local = RecordTimestamp::find_by_statement(stmt)
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(local) = local {
            // Skip if local is already newer than the incoming change timestamp.
            if local.updated_at > item.created_at {
                continue;
            }
        }

        result.push(item);
    }

    Ok(result)
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

fn resolve_delta<T>(upstream: T, downstream: T) -> Option<T>
where
    T: EntityTrait + ColumnTrait,
{
    todo!()
}
