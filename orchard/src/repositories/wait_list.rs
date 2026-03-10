use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use ulid::Ulid;

use crate::{
    adapters::wait_list::JoinWaitListRequest, entities::wait_list,
    errors::database_error::DatabaseError, repositories::base::Repository,
};

#[derive(Clone)]
pub struct WaitListRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for WaitListRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

pub(crate) trait WaitListRepositoryExt {
    async fn add_to_wait_list(
        &self,
        request: &JoinWaitListRequest,
    ) -> Result<wait_list::Model, DatabaseError>;

    async fn find_by_email(&self, email: &str) -> Result<Option<wait_list::Model>, DatabaseError>;

    async fn fetch_all(&self) -> Result<Vec<wait_list::Model>, DatabaseError>;
}

impl WaitListRepositoryExt for WaitListRepository {
    async fn add_to_wait_list(
        &self,
        request: &JoinWaitListRequest,
    ) -> Result<wait_list::Model, DatabaseError> {
        let entry = wait_list::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            first_name: Set(request.first_name.to_owned()),
            last_name: Set(request.last_name.to_owned()),
            email: Set(request.email.to_owned()),
            ..Default::default()
        };

        let entry = entry.insert(self.db_conn.as_ref()).await?;
        Ok(entry)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<wait_list::Model>, DatabaseError> {
        let entry = wait_list::Entity::find()
            .filter(wait_list::Column::Email.eq(email))
            .one(self.db_conn.as_ref())
            .await?;

        Ok(entry)
    }

    async fn fetch_all(&self) -> Result<Vec<wait_list::Model>, DatabaseError> {
        let entries = wait_list::Entity::find().all(self.db_conn.as_ref()).await?;

        Ok(entries)
    }
}
