use std::path::Path;
use std::sync::Arc;

use almond_kernel::utils::extract_env;
use axum_typed_multipart::TypedMultipart;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::adapters::authentication::SetNewPasswordRequest;
use crate::adapters::profile::UploadProfilePictureRequest;
use crate::adapters::users::PartialUserProfile;
use crate::config::AppConfig;
use crate::entities::users;
use crate::errors::database_error::DatabaseError;
use crate::errors::service_error::ServiceError;
use crate::repositories::user::{UserRepository, UserRepositoryTrait};
use crate::services::helper_service::{ServiceHelpers, ServiceHelpersTrait};

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
    user_helper_service: ServiceHelpers,
}

impl UserService {
    pub fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            user_repository: <UserRepository as crate::repositories::base::Repository>::init(
                db_conn,
            ),
            user_helper_service: ServiceHelpers::init(),
        }
    }
}

pub(crate) trait UserServiceTrait {
    async fn retrieve_information(
        &self,
        user_identifier: Uuid,
    ) -> Result<users::Model, DatabaseError>;

    async fn update_password(
        &self,

        request: &SetNewPasswordRequest,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError>;

    async fn update_profile_picture(
        &self,
        request: TypedMultipart<UploadProfilePictureRequest>,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError>;

    async fn update_profile(
        &self,
        request: &PartialUserProfile,
        user_identifier: &Uuid,
    ) -> Result<users::Model, ServiceError>;
}

impl UserServiceTrait for UserService {
    async fn retrieve_information(
        &self,
        user_identifier: Uuid,
    ) -> Result<users::Model, DatabaseError> {
        self.user_repository
            .retrieve_information(&user_identifier)
            .await
    }

    async fn update_password(
        &self,

        request: &SetNewPasswordRequest,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        let password_hash = self.user_helper_service.hash_password(&request.password)?;
        self.user_repository
            .update_password(user_identifier, &password_hash)
            .await?;

        Ok(())
    }

    async fn update_profile_picture(
        &self,
        TypedMultipart(UploadProfilePictureRequest { image }): TypedMultipart<
            UploadProfilePictureRequest,
        >,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        // tokio::task::spawn(async move {
        let file_name = image.metadata.file_name.clone().unwrap(); //TODO: undo unwrap

        let config = AppConfig::from_env().unwrap();
        let temp_dir = Path::new(&config.upload_path);
        let file_path = temp_dir.join(format!(
            "{time_stamp}_{file_name}",
            time_stamp = chrono::Local::now().timestamp()
        ));

        // create file object
        if let Err(err) = image.contents.persist(&file_path) {
            log::error!("error processing file due to {err}");
            return Err(ServiceError::OperationFailed);
        }

        // let private_key: String =
        //     extract_env("IMAGEKIT_PRIVATE_KEY").map_err(ServiceError::from).unwrap();
        // let public_key: String = extract_env("IMAGEKIT_PUBLIC_KEY").map_err(ServiceError::from).unwrap();

        // let client = ImagekitClient::new(&public_key, &private_key).map_err(|err| {
        //     log::error!("ImageKit client creation failed: {err}");
        //     ServiceError::OperationFailed
        // })?;

        // let url = client
        //     .upload_file(file_path, &file_name)
        //     .await
        //     .map_err(|err| {
        //         log::error!("MP3 upload failed: {err}");
        //         ServiceError::OperationFailed
        //     })
        //     .map(|res| res.url)?;

        // self.user_repository
        //     .set_avatar_url(user_identifier, &url)
        //     .await?;

        Ok(())
    }

    async fn update_profile(
        &self,
        request: &PartialUserProfile,
        user_identifier: &Uuid,
    ) -> Result<users::Model, ServiceError> {
        let profile = self
            .user_repository
            .update_profile(request, user_identifier)
            .await?;

        Ok(profile)
    }
}
