use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::adapters::wait_list::{JoinWaitListRequest, JoinWaitListResponse};
use crate::errors::database_error::DatabaseError;
use crate::errors::service_error::ServiceError;
use crate::repositories::base::Repository;
use crate::repositories::wait_list::{WaitListRepository, WaitListRepositoryExt};
use crate::services::helper_service::{ServiceHelpers, ServiceHelpersTrait};

#[derive(Clone)]
pub struct WaitListService {
    wait_list_repository: WaitListRepository,
}

impl WaitListService {
    pub fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            wait_list_repository: WaitListRepository::init(db_conn),
        }
    }
}

pub trait WaitListServiceTrait {
    async fn join_wait_list(
        &self,
        request: &JoinWaitListRequest,
    ) -> Result<JoinWaitListResponse, ServiceError>;
}

impl WaitListServiceTrait for WaitListService {
    async fn join_wait_list(
        &self,
        request: &JoinWaitListRequest,
    ) -> Result<JoinWaitListResponse, ServiceError> {
        let existing = self
            .wait_list_repository
            .find_by_email(&request.email)
            .await?;

        if existing.is_some() {
            return Err(ServiceError::DatabaseError(DatabaseError::DuplicateRecord));
        }

        let entry = self.wait_list_repository.add_to_wait_list(request).await?;

        let email = entry.email.clone();
        let first_name = entry.first_name.clone();

        tokio::task::spawn(async move {
            let service_helpers = ServiceHelpers::init();
            service_helpers
                .send_wait_list_confirmation_email(&email, &first_name)
                .await
                .unwrap_or_else(|err| {
                    log::error!("Failed to send wait list confirmation email: {err}");
                });
        });

        Ok(JoinWaitListResponse {
            identifier: entry.identifier,
            email: entry.email,
        })
    }
}
