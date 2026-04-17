use almond_kernel::entities;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncBookmark;

#[CustomFields]
impl SyncBookmark {
    async fn sync_bookmark(
        ctx: &Context<'_>,
        input: Vec<entities::bookmark::Model>,
    ) -> async_graphql::Result<bool> {
        unimplemented!()
    }
}
