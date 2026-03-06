use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use ulid::Ulid;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    adapters::{
        authentication::{CreateUserRequest, OnboardingRequest},
        repository::DatabaseInsertResult,
        users::PartialUserProfile,
    },
    entities::{backup_emails, users},
    errors::database_error::DatabaseError,
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct UserRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for UserRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self { db_conn: db_conn.to_owned() }
    }
}
pub(crate) trait UserRepositoryTrait {
    async fn find_by_identifier(&self, identifier: &Uuid) -> Option<users::Model>;

    async fn find_by_email(&self, email: &str) -> Option<users::Model>;

    async fn activate_account(&self, identifier: &Uuid) -> Result<(), DatabaseError>;

    async fn update_password(
        &self,
        identifier: &Uuid,
        new_password: &str,
    ) -> Result<(), DatabaseError>;

    async fn create_user(
        &self,
        user: &CreateUserRequest,
    ) -> Result<DatabaseInsertResult, DatabaseError>;

    async fn retrieve_information(&self, identifier: &Uuid) -> Result<users::Model, DatabaseError>;

    async fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> Result<(), DatabaseError>;

    async fn update_profile(
        &self,
        request: &PartialUserProfile,
        user_identifier: &Uuid,
    ) -> Result<users::Model, DatabaseError>;

    async fn onboard_user(
        &self,
        user_identifier: &Uuid,
        request: &OnboardingRequest,
    ) -> Result<(), DatabaseError>;

    async fn toggle_2fa(&self, user_identifier: &Uuid) -> Result<users::Model, DatabaseError>;

    async fn toggle_biometrics(
        &self,
        user_identifier: &Uuid,
    ) -> Result<users::Model, DatabaseError>;

    async fn add_backup_email(
        &self,
        user_identifier: &Uuid,
        backup_email: &str,
    ) -> Result<(), DatabaseError>;
}

impl UserRepositoryTrait for UserRepository {
    async fn create_user(
        &self,
        user: &CreateUserRequest,
    ) -> Result<DatabaseInsertResult, DatabaseError> {
        let normalized_email = user.email.to_lowercase().trim().to_string();

        let record = users::ActiveModel {
            identifier: Set(Uuid::new_v4()),
            email: Set(normalized_email),
            password: Set(user.password.clone()),
            is_active: Set(false),
            ..Default::default()
        };

        let inserted_user = record
            .insert(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(DatabaseInsertResult {
            identifier: inserted_user.identifier,
        })
    }

    async fn find_by_identifier(&self, identifier: &Uuid) -> Option<users::Model> {
        users::Entity::find()
            .filter(users::Column::Identifier.eq(identifier.to_owned()))
            .one(self.db_conn.as_ref())
            .await
            .ok()?
    }

    async fn find_by_email(&self, email: &str) -> Option<users::Model> {
        let normalized_email = email.to_lowercase().trim().to_string();

        users::Entity::find()
            .filter(users::Column::Email.eq(normalized_email))
            .one(self.db_conn.as_ref())
            .await
            .ok()?
    }

    async fn activate_account(&self, identifier: &Uuid) -> Result<(), DatabaseError> {
        let user = self
            .find_by_identifier(identifier)
            .await
            .ok_or(DatabaseError::RecordNotFound)?;
        let mut user: users::ActiveModel = user.into();
        user.is_active = Set(true);
        user.update(self.db_conn.as_ref()).await?;

        Ok(())
    }

    async fn update_password(
        &self,
        identifier: &Uuid,
        new_password: &str,
    ) -> Result<(), DatabaseError> {
        let user = self
            .find_by_identifier(identifier)
            .await
            .ok_or(DatabaseError::RecordNotFound)?;
        let mut user: users::ActiveModel = user.into();
        user.password = Set(new_password.to_string());
        user.update(self.db_conn.as_ref()).await?;
        Ok(())
    }

    async fn retrieve_information(&self, identifier: &Uuid) -> Result<users::Model, DatabaseError> {
        let user = self
            .find_by_identifier(identifier)
            .await
            .ok_or(DatabaseError::RecordNotFound)?;

        Ok(user)
    }

    async fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> Result<(), DatabaseError> {
        let user = self
            .find_by_identifier(user_identifier)
            .await
            .ok_or(DatabaseError::RecordNotFound)?;
        let mut user: users::ActiveModel = user.into();
        user.profile_picture = Set(Some(avatar_url.to_string()));

        user.update(self.db_conn.as_ref()).await?;

        Ok(())
    }

    async fn update_profile(
        &self,
        update: &PartialUserProfile,
        user_identifier: &Uuid,
    ) -> Result<users::Model, DatabaseError> {
        let user = self
            .find_by_identifier(user_identifier)
            .await
            .ok_or(DatabaseError::RecordNotFound)?;

        let mut user: users::ActiveModel = user.into();

        let PartialUserProfile {
            email,
            first_name,
            last_name,
            username,
        } = update;

        if let Some(val) = email {
            user.email = Set(val.to_owned());
        };

        if let Some(val) = first_name {
            user.first_name = Set(val.to_owned().into());
        };

        if let Some(val) = last_name {
            user.last_name = Set(val.to_owned().into());
        };

        if let Some(val) = username {
            user.username = Set(val.to_owned().into());
        };

        let update = user.update(self.db_conn.as_ref()).await?;

        Ok(update)
    }

    async fn onboard_user(
        &self,
        user_identifier: &Uuid,
        request: &OnboardingRequest,
    ) -> Result<(), DatabaseError> {
        let first_name = request.first_name.to_lowercase().trim().to_string();
        let last_name = request.last_name.to_lowercase().trim().to_string();
        let username = request
            .username
            .clone()
            .unwrap_or_else(|| format!("{}-{}", first_name, last_name))
            .to_lowercase()
            .trim()
            .to_string();

        let user = self
            .find_by_identifier(user_identifier)
            .await
            .ok_or(DatabaseError::RecordNotFound)?;

        let mut user: users::ActiveModel = user.into();

        user.username = Set(username.into());
        user.first_name = Set(first_name.into());
        user.last_name = Set(last_name.into());

        user.update(self.db_conn.as_ref()).await?;

        Ok(())
    }

    async fn toggle_2fa(&self, user_identifier: &Uuid) -> Result<users::Model, DatabaseError> {
        let user = self
            .find_by_identifier(user_identifier)
            .await
            .ok_or(DatabaseError::RecordNotFound)?;

        let prev_value = user.enable_biometrics;
        let mut user: users::ActiveModel = user.into();
        user.enable_2fa = Set(!prev_value);
        let update = user.update(self.db_conn.as_ref()).await?;

        Ok(update)
    }

    async fn toggle_biometrics(
        &self,
        user_identifier: &Uuid,
    ) -> Result<users::Model, DatabaseError> {
        let user = self
            .find_by_identifier(user_identifier)
            .await
            .ok_or(DatabaseError::RecordNotFound)?;

        let prev_value = user.enable_biometrics;
        let mut user: users::ActiveModel = user.into();
        user.enable_biometrics = Set(!prev_value);

        let update = user.update(self.db_conn.as_ref()).await?;

        Ok(update)
    }

    async fn add_backup_email(
        &self,
        user_identifier: &Uuid,
        backup_email: &str,
    ) -> Result<(), DatabaseError> {
        let backup_email = backup_emails::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            email: Set(backup_email.to_string()),
            user_identifier: Set(*user_identifier),
            ..Default::default()
        };

        backup_email.insert(self.db_conn.as_ref()).await?;
        Ok(())
    }
}
