use sea_orm::Database;
use seaography::async_graphql::{self, Context, InputObject};

use crate::entities::sync_queue;

pub async fn sync_todos(
    ctx: &Context<'_>,
    data: Vec<sync_queue::Model>,
) -> async_graphql::Result<bool> {
    let db = ctx.data::<Database>()?;

    dbg!("Syncing todos: {:#?}", data);

    Ok(true)
}
