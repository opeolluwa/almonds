use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use uuid::Uuid;

use crate::{
    adapters::{
        meta::RequestMeta,
        recycle_bin::{CreateRecycleBinEntry, RecycleBinItemType},
        workspace::{CreateWorkspace, UpdateWorkspace, hash_password, verify_password},
    },
    entities::workspaces,
    error::KernelError,
    repositories::{prelude::RecycleBinRepositoryExt, recycle_bin::RecycleBinRepository},
    utils::extract_req_meta,
};

#[derive(Debug, Clone)]
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

    async fn update_workspace(
        &self,
        identifier: &Uuid,
        payload: UpdateWorkspace,
    ) -> Result<workspaces::Model, KernelError>;

    async fn verify_workspace_password(
        &self,
        identifier: &Uuid,
        password: &str,
    ) -> Result<bool, KernelError>;

    async fn exists(&self, id: &Uuid) -> Result<bool, KernelError>;
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

        if model.is_default {
            return Err(KernelError::DbOperationError(
                "Cannot delete the default workspace".to_string(),
            ));
        }

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

    async fn update_workspace(
        &self,
        identifier: &Uuid,
        payload: UpdateWorkspace,
    ) -> Result<workspaces::Model, KernelError> {
        // If promoting to default, demote all others first
        if payload.is_default == Some(true) {
            workspaces::Entity::update_many()
                .col_expr(workspaces::Column::IsDefault, Expr::value(false))
                .exec(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        }

        let model = self.get_workspace_by_id(*identifier).await?;
        let mut active: workspaces::ActiveModel = model.into();

        if let Some(name) = payload.name {
            active.name = Set(name);
        }
        if let Some(description) = payload.description {
            active.description = Set(description);
        }
        if let Some(is_default) = payload.is_default {
            active.is_default = Set(is_default);
        }
        if let Some(is_hidden) = payload.is_hidden {
            active.is_hidden = Set(is_hidden);
        }
        if let Some(is_secured) = payload.is_secured {
            active.is_secured = Set(is_secured);
            // When disabling security, clear the hash
            if !is_secured {
                active.password_hash = Set(None);
            }
        }
        if let Some(password) = payload.password {
            if password.is_empty() {
                active.password_hash = Set(None);
            } else {
                let hash = hash_password(&password)
                    .map_err(|e| KernelError::DbOperationError(e.to_string()))?;
                active.password_hash = Set(Some(hash));
            }
        }
        active.updated_at = Set(Utc::now().fixed_offset());

        active
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn verify_workspace_password(
        &self,
        identifier: &Uuid,
        password: &str,
    ) -> Result<bool, KernelError> {
        let model = self.get_workspace_by_id(*identifier).await?;
        if !model.is_secured {
            return Ok(true);
        }
        match model.password_hash {
            Some(ref hash) => verify_password(password, hash)
                .map_err(|e| KernelError::DbOperationError(e.to_string())),
            None => Ok(false),
        }
    }

    async fn exists(&self, id: &Uuid) -> Result<bool, KernelError> {
        let result = self.get_workspace_by_id(id.to_owned()).await.ok();
        Ok(result.is_some())
    }
}
