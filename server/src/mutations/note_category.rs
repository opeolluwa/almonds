use almond_kernel::entities;
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncNoteCategory;

#[CustomFields]
impl SyncNoteCategory {
    async fn sync_note_category(
        _ctx: &Context<'_>,
        _input: Vec<entities::note_categories::Model>,
    ) -> async_graphql::Result<bool> {
        unimplemented!()
    }
}
