mod adapters;
mod commands;
mod errors;
mod state;
mod utils;

use std::sync::Arc;

use tauri::Manager;

use commands::sync_queue::{
    add_sync_queue_entry, count_sync_queue_entries, remove_sync_queue_entry, run_sync,
};
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

                std::fs::create_dir_all(&app_data_dir)
                    .expect("failed to create app data dir");

                let db_path = app_data_dir.join("almond.db");
                let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

                let kernel = almond_kernel::kernel::Kernel::new(&db_url)
                    .await
                    .expect("failed to initialize kernel");

                kernel
                    .run_migrations()
                    .await
                    .expect("failed to run migrations");

                let state = AppState {
                    conn: Arc::new(kernel.connection().clone()),
                };

                app_handle.manage(state);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_sync_queue_entry,
            remove_sync_queue_entry,
            count_sync_queue_entries,
            run_sync,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
