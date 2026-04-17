use almond_kernel::entities;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncNote;

#[CustomFields]
impl SyncNote {
    async fn sync_note(
        ctx: &Context<'_>,
        input: Vec<entities::notes::Model>,
    ) -> async_graphql::Result<bool> {
        unimplemented!()
    }
}
