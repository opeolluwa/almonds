mod m20260217_143820_create_snippet_table;
mod m20260217_150510_create_sync_queue_table;
mod m20260218_071539_create_ollama_conversation_prompt;
mod m20260218_071549_create_ollama_conversation_response;
mod m20260218_071617_create_ollama_conversation_history;
mod m20260218_110352_create_note_table;
mod m20260218_110353_create_note_category_table;
mod m20260218_171131_create_todo_table;
mod m20260218_204212_create_bookmarks_table;
mod m20260219_214114_add_due_time_to_todo;
mod m20260221_065202_create_reminder_table;
mod m20260221_065819_create_recycle_bin;
mod m20260221_065938_create_user_preference_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260217_143820_create_snippet_table::Migration),
            Box::new(m20260217_150510_create_sync_queue_table::Migration),
            Box::new(m20260218_071539_create_ollama_conversation_prompt::Migration),
            Box::new(m20260218_071549_create_ollama_conversation_response::Migration),
            Box::new(m20260218_071617_create_ollama_conversation_history::Migration),
            Box::new(m20260218_110352_create_note_table::Migration),
            Box::new(m20260218_110353_create_note_category_table::Migration),
            Box::new(m20260218_171131_create_todo_table::Migration),
            Box::new(m20260218_204212_create_bookmarks_table::Migration),
            Box::new(m20260219_214114_add_due_time_to_todo::Migration),
            Box::new(m20260221_065202_create_reminder_table::Migration),
            Box::new(m20260221_065819_create_recycle_bin::Migration),
            Box::new(m20260221_065938_create_user_preference_table::Migration),
        ]
    }
}
