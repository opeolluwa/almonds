use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    adapters::{
        meta::RequestMeta,
        recycle_bin::{CreateRecycleBinEntry, RecycleBinItemType},
        workspace::CreateWorkspace,
    },
    entities::workspaces,
    error::KernelError,
    repositories::{prelude::RecycleBinRepositoryExt, recycle_bin::RecycleBinRepository},
    utils::extract_req_meta,
};

pub struct WorkspaceRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait WorkspaceRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create_workspace(
        &self,
        workspace: CreateWorkspace,
    ) -> Result<workspaces::Model, KernelError>;

    async fn get_workspace_by_id(&self, id: Uuid) -> Result<workspaces::Model, KernelError>;

    async fn list_workspaces(&self) -> Result<Vec<workspaces::Model>, KernelError>;

    async fn delete_workspace(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError>;
}

#[async_trait]
impl WorkspaceRepositoryExt for WorkspaceRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create_workspace(
        &self,
        workspace: CreateWorkspace,
    ) -> Result<workspaces::Model, KernelError> {
        let active_model: workspaces::ActiveModel = workspace.into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn get_workspace_by_id(&self, id: Uuid) -> Result<workspaces::Model, KernelError> {
        workspaces::Entity::find()
            .filter(workspaces::Column::Identifier.eq(id))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
            .and_then(|opt| {
                opt.ok_or_else(|| {
                    KernelError::DbOperationError(format!("Workspace with id {} not found", id))
                })
            })
    }

    async fn list_workspaces(&self) -> Result<Vec<workspaces::Model>, KernelError> {
        workspaces::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete_workspace(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = workspaces::Entity::find()
            .filter(workspaces::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("workspace not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: RecycleBinItemType::Workspace,
                    workspace_identifier: None,
                    payload,
                },
                &Some(meta.clone()),
            )
            .await?;

        let result = workspaces::Entity::delete_by_id(*identifier)
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        log::info!("{:#?}", result);
        Ok(())
    }
}
