use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::{
    adapters::user_preference::{CreateUserPreference, UpdateUserPreference},
    entities::user_preference,
    error::KernelError,
};

pub struct UserPreferenceRepository {
    conn: Arc<DatabaseConnection>,
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
        Self { conn }
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
