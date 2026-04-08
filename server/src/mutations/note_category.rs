use entities::note_categories::{ActiveModel, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use seaography::{
    async_graphql::{self, Context},
    itertools::Itertools,
    CustomFields,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entities, errors::app_error::AppError, types::note_category::CreateNoteCategoryInput,
    utils::validator::format_validation_errors,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteCategory;

#[CustomFields]
impl CreateNoteCategory {
    async fn create_note_category(
        ctx: &Context<'_>,
        input: CreateNoteCategoryInput,
    ) -> async_graphql::Result<entities::note_categories::Model, AppError> {
        if let Err(err) = input.validate() {
            let better_error_message = format_validation_errors(err);
            return Err(AppError::OperationFailed(
                better_error_message.into_iter().join(","),
            ));
        }

        let db_conn = ctx
            .data::<DatabaseConnection>()
            .map_err(|err| AppError::InternalError(err.message))?;

        let active_model: ActiveModel = input.into();

        let model: Model = active_model
            .insert(db_conn)
            .await
            .map_err(|err: DbErr| AppError::GraphQLError(err.to_string()))?;

        Ok(model)
    }
}
