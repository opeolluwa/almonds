use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::{
        meta::RequestMeta,
        notes::{CreateNote, UpdateNote},
        recycle_bin::{CreateRecycleBinEntry, RecycleBinItemType},
    },
    entities::notes,
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
pub struct NotesRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait NotesRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateNote,
        meta: &Option<RequestMeta>,
    ) -> Result<notes::Model, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<notes::Model>, KernelError>;

    async fn find_all(&self, meta: &Option<RequestMeta>) -> Result<Vec<notes::Model>, KernelError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError>;

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<notes::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateNote,
        meta: &Option<RequestMeta>,
    ) -> Result<notes::Model, KernelError>;
}

#[async_trait]
impl NotesRepositoryExt for NotesRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn),
        }
    }

    async fn create(
        &self,
        payload: &CreateNote,
        meta: &Option<RequestMeta>,
    ) -> Result<notes::Model, KernelError> {
        let mut active_model: notes::ActiveModel = payload.to_owned().into();

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
    ) -> Result<Option<notes::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        notes::Entity::find()
            .filter(notes::Column::Identifier.eq(*identifier))
            .filter(notes::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(&self, meta: &Option<RequestMeta>) -> Result<Vec<notes::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        notes::Entity::find()
            .filter(notes::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = notes::Entity::find()
            .filter(notes::Column::Identifier.eq(*identifier))
            .filter(notes::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("note not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: RecycleBinItemType::Note,
                    payload,
                    workspace_identifier: model.workspace_identifier,
                },
                &Some(meta.clone()),
            )
            .await?;

        notes::Entity::delete_many()
            .filter(notes::Column::Identifier.eq(*identifier))
            .filter(notes::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<notes::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        notes::Entity::find()
            .limit(10)
            .filter(notes::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_asc(notes::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateNote,
        meta: &Option<RequestMeta>,
    ) -> Result<notes::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = notes::Entity::find()
            .filter(notes::Column::Identifier.eq(*identifier))
            .filter(notes::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("note not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(title.clone());
        }
        if let Some(content) = &payload.content {
            active_model.content = Set(content.clone());
        }
        if let Some(categories) = &payload.categories {
            active_model.categories = Set(serde_json::json!(categories).into());
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}

impl TransferRecord for NotesRepository {
    async fn transfer_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError> {
        let workspace = self
            .workspace_repository
            .get_workspace_by_id(*target_workspace_identifier)
            .await?;

        todo!()
    }
}

impl RecordExistInWorkspace for NotesRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, KernelError> {
        todo!()
    }
}

impl DuplicateRecord for NotesRepository {
    async fn duplicate_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError> {
        todo!()
    }
}
