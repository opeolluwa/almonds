use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{adapters::workspace::CreateWorkspace, entities::workspace, error::KernelError};

pub struct WorkspaceRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait WorkspaceRepositoryExt {
    async fn create_workspace(
        &self,
        workspace: CreateWorkspace,
    ) -> Result<workspace::Model, KernelError>;

    async fn get_workspace_by_id(&self, id: Uuid) -> Result<workspace::Model, KernelError>;

    async fn list_workspaces(&self) -> Result<Vec<workspace::Model>, KernelError>;
}

#[async_trait]
impl WorkspaceRepositoryExt for WorkspaceRepository {
    async fn create_workspace(
        &self,
        workspace: CreateWorkspace,
    ) -> Result<workspace::Model, KernelError> {
        let active_model: workspace::ActiveModel = workspace.into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn get_workspace_by_id(&self, id: Uuid) -> Result<workspace::Model, KernelError> {
        workspace::Entity::find()
            .filter(workspace::Column::Identifier.eq(id))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
            .and_then(|opt| {
                opt.ok_or_else(|| {
                    KernelError::DbOperationError(format!("Workspace with id {} not found", id))
                })
            })
    }

    async fn list_workspaces(&self) -> Result<Vec<workspace::Model>, KernelError> {
        workspace::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}
