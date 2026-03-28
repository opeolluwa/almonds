use entities::ollama_conversation_response::{ActiveModel, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use seaography::{
    async_graphql::{self, Context},
    itertools::Itertools,
    CustomFields,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entities, errors::app_error::AppError,
    types::ollama_conversation_response::CreateOllamaConversationResponseInput,
    utils::validator::format_validation_errors,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOllamaConversationResponse;

#[CustomFields]
impl CreateOllamaConversationResponse {
    async fn create_ollama_conversation_response(
        ctx: &Context<'_>,
        input: CreateOllamaConversationResponseInput,
    ) -> async_graphql::Result<entities::ollama_conversation_response::Model, AppError> {
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
