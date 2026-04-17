use almond_kernel::entities;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncNoteCategory;

#[CustomFields]
impl SyncNoteCategory {
    async fn sync_note_category(
        ctx: &Context<'_>,
        input: Vec<entities::note_categories::Model>,
    ) -> async_graphql::Result<bool> {
        unimplemented!()
    }
}
