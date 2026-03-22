use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static::lazy_static, Builder, BuilderContext};

use crate::{entities::*, mutations, types};

lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    schema_builder(&CONTEXT, database, depth, complexity).finish()
}

pub fn schema_builder(
    context: &'static BuilderContext,
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> SchemaBuilder {
    let mut builder = Builder::new(context, database.clone());
    builder = register_entity_modules(builder);
    builder = register_active_enums(builder);

    seaography::register_custom_inputs!(builder, [types::workspaces::CreateWorkspaceInput]);
    // seaography::register_custom_outputs!(builder, [types::PurchaseOrder, ..]);
    // seaography::register_complex_custom_outputs!(builder, [types::Rectangle, ..]);
    // seaography::register_custom_unions!(builder, [types::Shape, ..]);
    // seaography::register_custom_queries!(builder, [queries::Operations]);
    seaography::register_custom_mutations!(
        builder,
        [
            mutations::hello::SayHello,
            mutations::workspace::CreateWorkspace
        ]
    );

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .enable_uploading()
        .data(database)
}
