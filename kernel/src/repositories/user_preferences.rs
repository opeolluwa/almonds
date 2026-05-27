use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QuerySelect,
};
use uuid::Uuid;

#[cfg(feature = "sync_engine")]
use crate::types::EntitySyncResult;
use crate::{
    adapters::user_preferences::{CreateUserPreferences, UpdateUserPreferences},
    entities::{sync_queue, user_preferences},
    error::KernelError,
};

#[derive(Debug, Clone)]
pub struct UserPreferencesRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait UserPreferencesRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateUserPreferences,
    ) -> Result<user_preferences::Model, KernelError>;

    async fn get_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> Result<Option<user_preferences::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateUserPreferences,
    ) -> Result<user_preferences::Model, KernelError>;

    async fn extract_unsynced(&self) -> Result<Vec<user_preferences::Model>, KernelError>;

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), KernelError>;

    #[cfg(feature = "sync_engine")]
    async fn upsert_many(
        &self,
        models: Vec<user_preferences::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError>;
}

#[async_trait]
impl UserPreferencesRepositoryExt for UserPreferencesRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create(
        &self,
        payload: &CreateUserPreferences,
    ) -> Result<user_preferences::Model, KernelError> {
        let active_model: user_preferences::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn get_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> Result<Option<user_preferences::Model>, KernelError> {
        user_preferences::Entity::find()
            .filter(user_preferences::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateUserPreferences,
    ) -> Result<user_preferences::Model, KernelError> {
        let model = user_preferences::Entity::find()
            .filter(user_preferences::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| {
                KernelError::DbOperationError("user preferences not found".to_string())
            })?;

        let mut active_model = model.into_active_model();

        if let Some(first_name) = &payload.master_first_name {
            active_model.master_first_name = Set(first_name.clone());
        }
        if let Some(last_name) = &payload.master_last_name {
            active_model.master_last_name = Set(last_name.clone());
        }
        if let Some(email) = &payload.master_email {
            active_model.master_email = Set(email.clone());
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn extract_unsynced(&self) -> Result<Vec<user_preferences::Model>, KernelError> {
        let queue_entries = sync_queue::Entity::find()
            .filter(sync_queue::Column::TableName.eq("user_preferences"))
            .limit(25)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        let identifiers = queue_entries
            .iter()
            .map(|entry| {
                Uuid::parse_str(&entry.record_identifier)
                    .map_err(|err| KernelError::DbOperationError(err.to_string()))
            })
            .collect::<Result<Vec<Uuid>, KernelError>>()?;

        if identifiers.is_empty() {
            return Ok(Vec::new());
        }

        user_preferences::Entity::find()
            .filter(user_preferences::Column::Identifier.is_in(identifiers))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), KernelError> {
        sync_queue::Entity::delete_many()
            .filter(sync_queue::Column::TableName.eq("user_preferences"))
            .filter(sync_queue::Column::RecordIdentifier.is_in(identifiers))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    #[cfg(feature = "sync_engine")]
    async fn upsert_many(
        &self,
        models: Vec<user_preferences::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError> {
        let mut sync_results: Vec<EntitySyncResult> = Vec::new();
        for chunk in models.chunks(20) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|model| {
                    let conn = self.conn.clone();
                    let model = model.clone();
                    async move {
                        let identifier = model.identifier.to_string();
                        let op_result: Result<(), KernelError> = async {
                            let exists = user_preferences::Entity::find()
                                .filter(user_preferences::Column::Identifier.eq(model.identifier))
                                .one(conn.as_ref())
                                .await
                                .map_err(|err| KernelError::DbOperationError(err.to_string()))?
                                .is_some();

                            let active_model = model.into_active_model();

                            if exists {
                                active_model.update(conn.as_ref()).await.map_err(|err| {
                                    KernelError::DbOperationError(err.to_string())
                                })?;
                            } else {
                                active_model.insert(conn.as_ref()).await.map_err(|err| {
                                    KernelError::DbOperationError(err.to_string())
                                })?;
                            }
                            Ok(())
                        }
                        .await;
                        EntitySyncResult {
                            identifier,
                            success: op_result.is_ok(),
                            error_message: op_result.err().map(|e| e.to_string()),
                        }
                    }
                })
                .collect();

            let chunk_results = futures::future::join_all(futures).await;
            sync_results.extend(chunk_results);
        }
        Ok(sync_results)
    }
}
