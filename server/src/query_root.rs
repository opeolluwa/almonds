use almond_kernel::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static::lazy_static, Builder, BuilderContext};

use crate::entities::register_active_enums;
use crate::entities::register_entity_modules;
use crate::{mutations, types};

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

    seaography::register_custom_inputs!(
        builder,
        [
            types::workspaces::CreateWorkspaceInput,
            types::todo::CreateTodoInput,
            types::notes::CreateNoteInput,
            types::reminder::CreateReminderInput,
            types::snippets::CreateSnippetInput,
            types::bookmark::CreateBookmarkInput,
            types::recycle_bin::CreateRecycleBinItemInput,
            types::note_category::CreateNoteCategoryInput,
            types::ollama_conversation_history::CreateOllamaConversationHistoryInput,
            types::ollama_conversation_prompt::CreateOllamaConversationPromptInput,
            types::ollama_conversation_response::CreateOllamaConversationResponseInput
        ]
    );
    // seaography::register_custom_outputs!(builder, [types::PurchaseOrder, ..]);
    // seaography::register_complex_custom_outputs!(builder, [types::Rectangle, ..]);
    // seaography::register_custom_unions!(builder, [types::Shape, ..]);
    // seaography::register_custom_queries!(builder, [queries::Operations]);
    seaography::register_custom_mutations!(
        builder,
        [
            mutations::hello::SayHello,
            mutations::workspace::CreateWorkspace,
            mutations::todo::CreateTodo,
            mutations::notes::CreateNote,
            mutations::reminder::CreateReminder,
            mutations::snippets::CreateSnippet,
            mutations::bookmark::CreateBookmark,
            mutations::recycle_bin::CreateRecycleBinItem,
            mutations::note_category::CreateNoteCategory,
            mutations::ollama_conversation_history::CreateOllamaConversationHistory,
            mutations::ollama_conversation_prompt::CreateOllamaConversationPrompt,
            mutations::ollama_conversation_response::CreateOllamaConversationResponse
        ]
    );

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .enable_uploading()
        .data(database)
}
