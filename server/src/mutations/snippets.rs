use almond_kernel::entities;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncSnippet;

#[CustomFields]
impl SyncSnippet {
    async fn sync_snippet(
        ctx: &Context<'_>,
        input: Vec<entities::snippets::Model>,
    ) -> async_graphql::Result<bool> {
        unimplemented!()
    }
}
