use std::str::FromStr;
use std::sync::Arc;

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use crate::entities::one_time_passwords;
use crate::errors::database_error::DatabaseError;
use crate::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct OtpRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for OtpRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

#[allow(dead_code)]
pub(crate) trait OtpRepositoryExt {
    async fn new_with_user(&self, user_identifier: &str, code: &str) -> Result<(), DatabaseError>;

    async fn find_latest_by_user(
        &self,
        user_identifier: &Uuid,
    ) -> Result<Option<one_time_passwords::Model>, DatabaseError>;

    async fn find_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> Result<Option<one_time_passwords::Model>, DatabaseError>;

    async fn delete_by_identifier(&self, identifier: &Uuid) -> Result<(), DatabaseError>;
}

impl OtpRepositoryExt for OtpRepository {
    async fn new_with_user(&self, user_identifier: &str, code: &str) -> Result<(), DatabaseError> {
        let otp_identifier = Uuid::new_v4();
        let user_identifier = Uuid::from_str(user_identifier)
            .map_err(|err| DatabaseError::OperationFailed(err.to_string()))?;

        let otp = one_time_passwords::ActiveModel {
            identifier: Set(otp_identifier),
            code: Set(code.to_string()), //TODO: hash this code
            user_identifier: Set(user_identifier),
            ..Default::default()
        };
        otp.insert(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(())
    }

    async fn find_latest_by_user(
        &self,
        user_identifier: &Uuid,
    ) -> Result<Option<one_time_passwords::Model>, DatabaseError> {
        let otp = one_time_passwords::Entity::find()
            .filter(one_time_passwords::Column::UserIdentifier.eq(user_identifier.to_owned()))
            .order_by_desc(one_time_passwords::Column::CreatedAt)
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(otp)
    }

    async fn delete_by_identifier(&self, identifier: &Uuid) -> Result<(), DatabaseError> {
        let otp = one_time_passwords::Entity::find()
            .filter(one_time_passwords::Column::Identifier.eq(identifier.to_owned()))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        if otp.is_none() {
            return Err(DatabaseError::RecordNotFound);
        }

        let Some(otp) = otp else {
            return Err(DatabaseError::RecordNotFound);
        };

        let otp: one_time_passwords::ActiveModel = otp.into();
        otp.delete(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(())
    }

    async fn find_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> Result<Option<one_time_passwords::Model>, DatabaseError> {
        let otp = one_time_passwords::Entity::find()
            .filter(one_time_passwords::Column::Identifier.eq(identifier.to_owned()))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(otp)
    }
}
