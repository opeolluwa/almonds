use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use sea_orm::prelude::Date;
use uuid::Uuid;

use crate::{
    adapters::todo::{CreateTodo, TodoPriority, UpdateTodo},
    entities::todo,
    error::KernelError,
};

pub struct TodoRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait TodoRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create_todo(&self, payload: &CreateTodo) -> Result<todo::Model, KernelError>;

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<todo::Model>, KernelError>;

    async fn find_all(&self) -> Result<Vec<todo::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateTodo,
    ) -> Result<todo::Model, KernelError>;

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError>;

    async fn change_priority(
        &self,
        identifier: &Uuid,
        priority: &TodoPriority,
    ) -> Result<todo::Model, KernelError>;

    async fn update_due_date(
        &self,
        identifier: &Uuid,
        due_date: Option<Date>,
    ) -> Result<todo::Model, KernelError>;

    async fn mark_done(
        &self,
        identifier: &Uuid,
        done: bool,
    ) -> Result<todo::Model, KernelError>;
}

#[async_trait]
impl TodoRepositoryExt for TodoRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create_todo(&self, payload: &CreateTodo) -> Result<todo::Model, KernelError> {
        let active_model: todo::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<todo::Model>, KernelError> {
        todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<todo::Model>, KernelError> {
        todo::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateTodo,
    ) -> Result<todo::Model, KernelError> {
        let model = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("todo not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(title.clone());
        }
        if let Some(description) = &payload.description {
            active_model.description = Set(Some(description.clone()));
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError> {
        todo::Entity::delete_many()
            .filter(todo::Column::Identifier.eq(*identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn change_priority(
        &self,
        identifier: &Uuid,
        priority: &TodoPriority,
    ) -> Result<todo::Model, KernelError> {
        let model = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("todo not found".to_string()))?;

        let mut active_model = model.into_active_model();
        active_model.priority = Set(priority.to_string());
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update_due_date(
        &self,
        identifier: &Uuid,
        due_date: Option<Date>,
    ) -> Result<todo::Model, KernelError> {
        let model = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("todo not found".to_string()))?;

        let mut active_model = model.into_active_model();
        active_model.due_date = Set(due_date);
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn mark_done(
        &self,
        identifier: &Uuid,
        done: bool,
    ) -> Result<todo::Model, KernelError> {
        let model = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("todo not found".to_string()))?;

        let mut active_model = model.into_active_model();
        active_model.done = Set(done);
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}
