#[derive(InputObject)]
pub struct TodoSyncInput {
    pub identifier: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
    pub updated_at: String,
    pub operation: String,
}

pub async fn sync_todos(ctx: &Context<'_>, data: Vec<TodoSyncInput>) -> Result<bool> {
    let db = ctx.data::<Database>()?;

    for item in data {
        match item.operation.as_str() {
            "CREATE" | "UPDATE" => {
                // UPSERT logic
                Todo::upsert(item).exec(db).await?;
            }
            "DELETE" => {
                Todo::delete_by_id(item.identifier).exec(db).await?;
            }
            _ => {}
        }
    }

    Ok(true)
}
