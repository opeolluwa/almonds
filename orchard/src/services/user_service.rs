use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::adapters::dto::user::UserDto;
use crate::errors::user_service_error::UserServiceError;
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};

// use super::user_helper_service::{UserHelperService, UserHelperServiceTrait};

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            user_repository: UserRepository::init(pool),
            // user_helper_service: UserHelperService::init(),
        }
    }
}

pub(crate) trait UserServiceTrait {
    async fn retrieve_information(
        &self,
        user_identifier: Uuid,
    ) -> Result<UserDto, UserServiceError>;
}

impl UserServiceTrait for UserService {
    async fn retrieve_information(
        &self,
        user_identifier: Uuid,
    ) -> Result<UserDto, UserServiceError> {
        self.user_repository
            .retrieve_information(&user_identifier)
            .await
    }
}
