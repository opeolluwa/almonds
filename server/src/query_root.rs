use almond_kernel::entities::{register_active_enums, register_entity_modules};
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{
    async_graphql, lazy_static::lazy_static, Builder, BuilderContext, DecimalLibrary, TimeLibrary,
    TypesMapConfig,
};

use crate::{mutations, types};

lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext {
        types: TypesMapConfig {
            time_library: TimeLibrary::Chrono,
            decimal_library: DecimalLibrary::Decimal,
            ..Default::default()
        },
        ..Default::default()
    };
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

    seaography::register_custom_inputs!(
        builder,
        [
            types::bookmark::SyncBookmarkInput,
            types::note::SyncNoteInput,
            types::snippet::SyncSnippetInput,
            types::todo::SyncTodoInput,
            types::reminder::SyncReminderInput,
            types::workspace::SyncWorkspaceInput,
            types::recycle_bin::SyncRecycleBinInput,
            types::user_preference::SyncUserPreferenceInput,
        ]
    );


    seaography::register_custom_outputs!(builder, [almond_kernel::sync_engine::EntitySyncResult,]);

    seaography::register_custom_mutations!(
        builder,
        [
            mutations::preflight::Preflight,
            mutations::sync_queue::SyncQueue,
            mutations::bookmark::SyncBookmark,
            mutations::notes::SyncNote,
            mutations::note_category::SyncNoteCategory,
            mutations::snippets::SyncSnippet,
            mutations::todo::SyncTodo,
            mutations::reminder::SyncReminder,
            mutations::workspace::SyncWorkspace,
            mutations::recycle_bin::SyncRecycleBinItem,
        ]
    );

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .enable_uploading()
        .data(database)
}
