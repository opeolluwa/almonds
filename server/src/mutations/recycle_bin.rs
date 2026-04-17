use almond_kernel::entities;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncRecycleBinItem;

#[CustomFields]
impl SyncRecycleBinItem {
    async fn sync_recycle_bin_item(
        ctx: &Context<'_>,
        input: Vec<entities::recycle_bin::Model>,
    ) -> async_graphql::Result<bool> {
        unimplemented!()
    }
}
