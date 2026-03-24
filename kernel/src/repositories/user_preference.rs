use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::repositories::{
    prelude::WorkspaceRepositoryExt,
    workspace::WorkspaceRepository,
    workspace_manager::{DuplicateRecord, RecordExistInWorkspace, TransferRecord},
};
use crate::{
    adapters::user_preference::{CreateUserPreference, UpdateUserPreference},
    entities::user_preference,
    error::KernelError,
};

pub struct UserPreferenceRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait UserPreferenceRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateUserPreference,
    ) -> Result<user_preference::Model, KernelError>;

    async fn get(&self) -> Result<Option<user_preference::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateUserPreference,
    ) -> Result<user_preference::Model, KernelError>;
}

#[async_trait]
impl UserPreferenceRepositoryExt for UserPreferenceRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn.clone()),
        }
    }

    async fn create(
        &self,
        payload: &CreateUserPreference,
    ) -> Result<user_preference::Model, KernelError> {
        let active_model: user_preference::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn get(&self) -> Result<Option<user_preference::Model>, KernelError> {
        user_preference::Entity::find()
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateUserPreference,
    ) -> Result<user_preference::Model, KernelError> {
        let model = user_preference::Entity::find()
            .filter(user_preference::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| {
                KernelError::DbOperationError("user preference not found".to_string())
            })?;

        let mut active_model = model.into_active_model();

        if let Some(first_name) = &payload.first_name {
            active_model.first_name = Set(first_name.clone());
        }
        if let Some(last_name) = &payload.last_name {
            active_model.last_name = Set(last_name.clone());
        }
        if let Some(email) = &payload.email {
            active_model.email = Set(email.clone());
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}
#[async_trait::async_trait]

impl TransferRecord for UserPreferenceRepository {
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
            return Err(KernelError::BookmarkNotFound(record_identifier.to_string()));
        }

        let Some(record) = user_preference::Entity::find()
            .filter(user_preference::Column::Identifier.eq(*record_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
        else {
            return Err(KernelError::BookmarkNotFound(record_identifier.to_string()));
        };

        let mut active_model = record.into_active_model();

        active_model.updated_at = Set(Utc::now().fixed_offset());
        // active_model.workspace_identifier = Set(Some(*target_workspace_identifier));

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}
#[async_trait::async_trait]

impl RecordExistInWorkspace for UserPreferenceRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        _workspace_identifier: &Uuid,
    ) -> Result<bool, KernelError> {
        let record = user_preference::Entity::find()
            .filter(user_preference::Column::Identifier.eq(*record_identifier))
            // .filter(user_preference::Column::WorkspaceIdentifier.eq(*workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(record.is_some())
    }
}
#[async_trait::async_trait]

impl DuplicateRecord for UserPreferenceRepository {
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

        let Some(record) = user_preference::Entity::find()
            .filter(user_preference::Column::Identifier.eq(*record_identifier))
            // .filter(user_preference::Column::WorkspaceIdentifier.eq(*previous_workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
        else {
            return Err(KernelError::BookmarkNotFound(record_identifier.to_string()));
        };

        let mut new_record = record.into_active_model();

        new_record.identifier = Set(Uuid::new_v4());
        // new_record.workspace_identifier = Set(Some(*target_workspace_identifier));//TODO: use workspace in user_preference table
        new_record.created_at = Set(Utc::now().fixed_offset());
        new_record.updated_at = Set(Utc::now().fixed_offset());

        new_record
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}
