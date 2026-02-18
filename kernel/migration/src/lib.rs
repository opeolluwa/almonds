mod m20260217_143820_create_snippet_table;
mod m20260217_150510_create_sync_queue_table;
mod m20260218_071539_create_ollama_conversation_prompt;
mod m20260218_071549_create_ollama_conversation_response;
mod m20260218_071617_create_ollama_conversation_history;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260217_143820_create_snippet_table::Migration),
            Box::new(m20260217_150510_create_sync_queue_table::Migration),
            Box::new(m20260218_071617_create_ollama_conversation_history::Migration),
            Box::new(m20260218_071539_create_ollama_conversation_prompt::Migration),
            Box::new(m20260218_071549_create_ollama_conversation_response::Migration),
        ]
    }
}
