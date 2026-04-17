use almond_kernel::entities;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncReminder;

#[CustomFields]
impl SyncReminder {
    async fn sync_reminder(
        ctx: &Context<'_>,
        input: Vec<entities::reminder::Model>,
    ) -> async_graphql::Result<bool> {
        unimplemented!()
    }
}
