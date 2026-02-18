mod adapters;
mod commands;
mod errors;
mod state;
mod utils;

use std::sync::Arc;

use almond_kernel::repositories::{
    snippets::{SnippetRepository, SnippetRepositoryExt},
    sync_queue::{SyncQueueRepository, SyncQueueRepositoryExt},
};
use tauri::Manager;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let app_data_dir = app_handle
                    .path()
                    .app_data_dir()
                    .expect("failed to resolve app data dir");

                std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");

                let db_path = app_data_dir.join("almond.db");
                let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

                let kernel = almond_kernel::kernel::Kernel::new(&db_url)
                    .await
                    .expect("failed to initialize kernel");

                kernel
                    .run_migrations()
                    .await
                    .expect("failed to run migrations");

                let conn = Arc::new(kernel.connection().clone());
                let snippet_repository = SnippetRepository::new(conn.clone());
                let sync_queue_repository = SyncQueueRepository::new(conn.clone());

                let state = AppState {
                    snippet_repository,
                    sync_queue_repository,
                };

                app_handle.manage(state);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::snippets::create_snippet,
            commands::snippets::get_snippet,
            commands::snippets::get_all_snippets,
            commands::snippets::delete_snippet,
            commands::snippets::update_snippet,
            commands::snippets::get_recently_added_snippet,
            commands::sync_queue::add_sync_queue_entry,
            commands::sync_queue::remove_sync_queue_entry,
            commands::sync_queue::count_sync_queue_entries,
            commands::sync_queue::run_sync,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
