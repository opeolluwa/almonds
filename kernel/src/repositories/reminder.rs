use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

#[cfg(feature = "postgres")]
use crate::entities::sea_orm_active_enums::ItemType;
#[cfg(feature = "sqlite")]
use crate::enums::ItemType;
#[cfg(feature = "sync_engine")]
use crate::types::EntitySyncResult;
use crate::{
    adapters::{
        meta::RequestMeta,
        recycle_bin::CreateRecycleBinEntry,
        reminder::{CreateReminder, UpdateReminder},
    },
    entities::reminder,
    error::KernelError,
    repositories::{
        prelude::WorkspaceRepositoryExt,
        recycle_bin::{RecycleBinRepository, RecycleBinRepositoryExt},
        workspace::WorkspaceRepository,
        workspace_manager::{DuplicateRecord, RecordExistInWorkspace, TransferRecord},
    },
    utils::extract_req_meta,
};

#[derive(Debug, Clone)]
pub struct ReminderRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait ReminderRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateReminder,
        meta: &Option<RequestMeta>,
    ) -> Result<reminder::Model, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<reminder::Model>, KernelError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<reminder::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateReminder,
        meta: &Option<RequestMeta>,
    ) -> Result<reminder::Model, KernelError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError>;

    #[cfg(feature = "sync_engine")]
    async fn upsert_many(
        &self,
        models: Vec<reminder::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError>;
}

#[async_trait]
impl ReminderRepositoryExt for ReminderRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn),
        }
    }

    async fn create(
        &self,
        payload: &CreateReminder,
        meta: &Option<RequestMeta>,
    ) -> Result<reminder::Model, KernelError> {
        let mut active_model: reminder::ActiveModel = payload.to_owned().into();

        if let Some(meta) = meta {
            active_model.workspace_identifier = Set(Some(meta.workspace_identifier));
        } else {
            return Err(KernelError::DbOperationError(
                "workspace identifier is required".into(),
            ));
        };
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<reminder::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        reminder::Entity::find()
            .filter(reminder::Column::Identifier.eq(*identifier))
            .filter(reminder::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<reminder::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        reminder::Entity::find()
            .filter(reminder::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateReminder,
        meta: &Option<RequestMeta>,
    ) -> Result<reminder::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = reminder::Entity::find()
            .filter(reminder::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(reminder::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("reminder not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(title.clone());
        }
        if let Some(description) = &payload.description {
            active_model.description = Set(Some(description.clone()));
        }
        if let Some(recurring) = payload.recurring {
            active_model.recurring = Set(recurring);
        }
        if let Some(recurrence_rule) = &payload.recurrence_rule {
            active_model.recurrence_rule = Set(Some(recurrence_rule.clone()));
        }
        if let Some(alarm_sound) = &payload.alarm_sound {
            active_model.alarm_sound = Set(Some(alarm_sound.clone()));
        }
        if let Some(remind_at) = payload.remind_at {
            active_model.remind_at = Set(remind_at);
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = reminder::Entity::find()
            .filter(reminder::Column::Identifier.eq(*identifier))
            .filter(reminder::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("reminder not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: ItemType::Reminder,
                    workspace_identifier: model.workspace_identifier,
                    payload,
                },
                &Some(meta.clone()),
            )
            .await?;

        reminder::Entity::delete_many()
            .filter(reminder::Column::Identifier.eq(*identifier))
            .filter(reminder::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    #[cfg(feature = "sync_engine")]
    async fn upsert_many(
        &self,
        models: Vec<reminder::Model>,
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
                            let exists = reminder::Entity::find()
                                .filter(reminder::Column::Identifier.eq(model.identifier))
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
#[async_trait::async_trait]

impl TransferRecord for ReminderRepository {
    async fn transfer_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(KernelError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(KernelError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        if !self
            .record_exists_in_workspace(record_identifier, previous_workspace_identifier)
            .await?
        {
            return Err(KernelError::ReminderNotFound(record_identifier.to_string()));
        }

        let Some(record) = reminder::Entity::find()
            .filter(reminder::Column::Identifier.eq(*record_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
        else {
            return Err(KernelError::BookmarkNotFound(record_identifier.to_string()));
        };

        let mut active_model = record.into_active_model();

        active_model.updated_at = Set(Utc::now().fixed_offset());
        active_model.workspace_identifier = Set(Some(*target_workspace_identifier));

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}
#[async_trait::async_trait]

impl RecordExistInWorkspace for ReminderRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, KernelError> {
        let record = reminder::Entity::find()
            .filter(reminder::Column::Identifier.eq(*record_identifier))
            .filter(reminder::Column::WorkspaceIdentifier.eq(*workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(record.is_some())
    }
}
#[async_trait::async_trait]

impl DuplicateRecord for ReminderRepository {
    async fn duplicate_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(KernelError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(KernelError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        let Some(record) = reminder::Entity::find()
            .filter(reminder::Column::Identifier.eq(*record_identifier))
            .filter(reminder::Column::WorkspaceIdentifier.eq(*previous_workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
        else {
            return Err(KernelError::ReminderNotFound(record_identifier.to_string()));
        };

        let mut new_record = record.into_active_model();

        new_record.identifier = Set(Uuid::new_v4());
        new_record.workspace_identifier = Set(Some(*target_workspace_identifier));
        new_record.created_at = Set(Utc::now().fixed_offset());
        new_record.updated_at = Set(Utc::now().fixed_offset());

        new_record
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}
