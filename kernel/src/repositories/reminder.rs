use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::{
    adapters::{
        recycle_bin::{CreateRecycleBinEntry, RecycleBinItemType},
        reminder::{CreateReminder, UpdateReminder},
    },
    entities::reminder,
    error::KernelError,
    repositories::recycle_bin::{RecycleBinRepository, RecycleBinRepositoryExt},
};

pub struct ReminderRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait ReminderRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(&self, payload: &CreateReminder) -> Result<reminder::Model, KernelError>;

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<reminder::Model>, KernelError>;

    async fn find_all(&self) -> Result<Vec<reminder::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateReminder,
    ) -> Result<reminder::Model, KernelError>;

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError>;
}

#[async_trait]
impl ReminderRepositoryExt for ReminderRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create(&self, payload: &CreateReminder) -> Result<reminder::Model, KernelError> {
        let active_model: reminder::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<reminder::Model>, KernelError> {
        reminder::Entity::find()
            .filter(reminder::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<reminder::Model>, KernelError> {
        reminder::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateReminder,
    ) -> Result<reminder::Model, KernelError> {
        let model = reminder::Entity::find()
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

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError> {
        let model = reminder::Entity::find()
            .filter(reminder::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("reminder not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        RecycleBinRepository::new(self.conn.clone())
            .store(&CreateRecycleBinEntry {
                item_id: model.identifier,
                item_type: RecycleBinItemType::Reminder,
                payload,
            })
            .await?;

        reminder::Entity::delete_many()
            .filter(reminder::Column::Identifier.eq(*identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }
}
