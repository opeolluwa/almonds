use entities::note_categories::ActiveModel;
use sea_orm::ActiveValue::Set;
use seaography::{
    async_graphql::{self},
    CustomInputType,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::entities;

#[derive(Debug, Serialize, Deserialize, Validate, CustomInputType)]
pub struct CreateNoteCategoryInput {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Label must be between 1 and 100 characters"
    ))]
    pub label: String,

    pub description: Option<String>,
}

impl Into<ActiveModel> for CreateNoteCategoryInput {
    fn into(self) -> entities::note_categories::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            label: Set(self.label),
            description: Set(self.description),
        }
    }
}
