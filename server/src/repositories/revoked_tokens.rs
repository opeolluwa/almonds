use std::sync::Arc;

use chrono::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::entities::revoked_tokens;
use crate::errors::database_error::DatabaseError;
use crate::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct TokenBlacklistRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for TokenBlacklistRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

pub(crate) trait TokenBlacklistRepositoryTrait {
    async fn revoke_token(
        &self,
        jti: &Uuid,
        user_identifier: &Uuid,
        expires_at: DateTime<chrono::FixedOffset>,
    ) -> Result<(), DatabaseError>;

    async fn is_revoked(&self, jti: &Uuid) -> Result<bool, DatabaseError>;
}

impl TokenBlacklistRepositoryTrait for TokenBlacklistRepository {
    async fn revoke_token(
        &self,
        jti: &Uuid,
        user_identifier: &Uuid,
        expires_at: DateTime<chrono::FixedOffset>,
    ) -> Result<(), DatabaseError> {
        let record = revoked_tokens::ActiveModel {
            identifier: Set(Uuid::new_v4()),
            jti: Set(*jti),
            user_identifier: Set(*user_identifier),
            expires_at: Set(expires_at),
            revoked_at: Set(chrono::Utc::now().fixed_offset()),
        };

        record
            .insert(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn is_revoked(&self, jti: &Uuid) -> Result<bool, DatabaseError> {
        let record = revoked_tokens::Entity::find()
            .filter(revoked_tokens::Column::Jti.eq(*jti))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;
        Ok(record.is_some())
    }
}
