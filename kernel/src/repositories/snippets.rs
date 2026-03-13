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
        recycle_bin::{CreateRecycleBinEntry, RecycleBinItemType},
        snippets::{CreateSnippet, UpdateSnippet},
    },
    entities::snippets,
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
pub struct SnippetRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait SnippetRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateSnippet,
        meta: &Option<RequestMeta>,
    ) -> Result<snippets::Model, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<snippets::Model>, KernelError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<snippets::Model>, KernelError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError>;

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<snippets::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateSnippet,
        meta: &Option<RequestMeta>,
    ) -> Result<snippets::Model, KernelError>;
}

#[async_trait]
impl SnippetRepositoryExt for SnippetRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn),
        }
    }

    async fn create(
        &self,
        payload: &CreateSnippet,
        meta: &Option<RequestMeta>,
    ) -> Result<snippets::Model, KernelError> {
        let mut active_model: snippets::ActiveModel = payload.to_owned().into();

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
    ) -> Result<Option<snippets::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        snippets::Entity::find()
            .filter(snippets::Column::Identifier.eq(*identifier))
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<snippets::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        snippets::Entity::find()
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
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

        let model = snippets::Entity::find()
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(snippets::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("snippet not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: RecycleBinItemType::Snippet,
                    workspace_identifier: model.workspace_identifier,
                    payload,
                },
                &Some(meta.clone()),
            )
            .await?;

        snippets::Entity::delete_many()
            .filter(snippets::Column::Identifier.eq(*identifier))
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<snippets::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        snippets::Entity::find()
            .limit(10)
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_asc(snippets::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateSnippet,
        meta: &Option<RequestMeta>,
    ) -> Result<snippets::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = snippets::Entity::find()
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(snippets::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("snippet not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(Some(title.clone()));
        }
        if let Some(language) = &payload.language {
            active_model.language = Set(Some(language.clone()));
        }
        if let Some(code) = &payload.code {
            active_model.code = Set(code.clone());
        }
        if let Some(description) = &payload.description {
            active_model.description = Set(Some(description.clone()));
        }
        if let Some(is_pinned) = payload.is_pinned {
            active_model.is_pinned = Set(is_pinned);
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}

impl TransferRecord for SnippetRepository {
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

impl RecordExistInWorkspace for SnippetRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, KernelError> {
        todo!()
    }
}

impl DuplicateRecord for SnippetRepository {
    async fn duplicate_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError> {
        todo!()
    }
}
