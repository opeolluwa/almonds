//! Auto generated module list
pub mod backup_emails;
pub mod bookmark;
pub mod countries;
pub mod note_categories;
pub mod notes;
pub mod notifications;
pub mod ollama_conversation_history;
pub mod ollama_conversation_prompt;
pub mod ollama_conversation_response;
pub mod one_time_passwords;
pub mod prelude;
pub mod recycle_bin;
pub mod reference_emails;
pub mod reminder;
pub mod snippets;
pub mod sync_queue;
pub mod todo;
pub mod user_preference;
pub mod users;
pub mod wait_list;
pub mod workspaces;

seaography::register_entity_modules!([
    bookmark,
    note_categories,
    notes,
    ollama_conversation_history,
    ollama_conversation_prompt,
    ollama_conversation_response,
    recycle_bin,
    reminder,
    snippets,
    sync_queue,
    todo,
    user_preference,
    workspaces,
]);
