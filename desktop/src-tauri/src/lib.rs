mod adapters;
mod commands;
mod errors;
mod scheduler;
mod state;
mod utils;

use std::sync::Arc;

use tauri::Manager;

// use tauri_plugin_decorum::WebviewWindowExt;
use crate::state::alarm::AlarmState;
use crate::state::app::AppState;
use crate::state::scheduler::SchedulerState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    builder
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        // .plugin(tauri_plugin_decorum::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // let main_window = app.get_webview_window("main").unwrap();
            // main_window.create_overlay_titlebar().unwrap();

            // Some macOS-specific helpers
            // #[cfg(target_os = "macos")]
            // {
            //     // Set a custom inset to the traffic lights
            //     main_window.set_traffic_lights_inset(12.0, 16.0).unwrap();

            //     // Make window transparent without privateApi
            //     main_window.make_transparent().unwrap();

            //     // Set window level
            //     // NSWindowLevel: https://developer.apple.com/documentation/appkit/nswindowlevel
            //     // main_window.set_window_level(25).unwrap();
            // }

            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let app_data_dir = app_handle
                    .path()
                    .app_data_dir()
                    .expect("failed to resolve app data dir");

                std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");

                let db_path = match std::env::var("ALMONDS_DB_PATH") {
                    Ok(path) => std::path::PathBuf::from(path),
                    Err(_) => app_data_dir.join("almonds.db"),
                };

                let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
                dbg!("Database URL: {:?}", &db_path);
                let kernel = almond_kernel::kernel::DataEngine::new(&db_url)
                    .await
                    .expect("failed to initialize kernel");

                kernel
                    .run_migrations()
                    .await
                    .expect("failed to run migrations");

                let conn = Arc::new(kernel.connection().clone());

                let state = AppState::new(conn).await;

                app_handle.manage(state);
                app_handle.manage(AlarmState::new());
                app_handle.manage(SchedulerState::new());
            });

            // Spawn the cron-style reminder scheduler.
            let scheduler_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                scheduler::run(scheduler_handle, None).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::alarm::list_alarm_sounds,
            commands::alarm::play_alarm_sound,
            commands::alarm::set_alarm_settings,
            commands::alarm::stop_alarm_sound,
            commands::bookmarks::create_bookmark,
            commands::bookmarks::delete_bookmark,
            commands::bookmarks::duplicate_bookmark,
            commands::bookmarks::get_all_bookmarks,
            commands::bookmarks::get_bookmark,
            commands::bookmarks::get_bookmarks_by_tag,
            commands::bookmarks::get_recently_added_bookmarks,
            commands::bookmarks::transfer_bookmark,
            commands::bookmarks::update_bookmark,
            commands::notes::create_note,
            commands::notes::delete_note,
            commands::notes::duplicate_note,
            commands::notes::get_all_notes,
            commands::notes::get_note,
            commands::notes::get_recently_added_notes,
            commands::notes::transfer_note,
            commands::notes::update_note,
            commands::notes::export_notes_as_pdf,
            commands::reminder::create_reminder,
            commands::reminder::delete_reminder,
            commands::reminder::duplicate_reminder,
            commands::reminder::get_all_reminders,
            commands::reminder::get_reminder,
            commands::reminder::transfer_reminder,
            commands::reminder::update_reminder,
            commands::recycle_bin::create_recycle_bin_entry,
            commands::recycle_bin::get_all_recycle_bin_entries,
            commands::recycle_bin::get_recycle_bin_entry,
            commands::recycle_bin::get_recycle_bin_entries_by_type,
            commands::recycle_bin::purge_all_recycle_bin_entries,
            commands::recycle_bin::purge_recycle_bin_entry,
            commands::snippets::create_snippet,
            commands::snippets::delete_snippet,
            commands::snippets::duplicate_snippet,
            commands::snippets::get_all_snippets,
            commands::snippets::get_recently_added_snippet,
            commands::snippets::get_snippet,
            commands::snippets::transfer_snippet,
            commands::snippets::update_snippet,
            commands::sync_queue::add_sync_queue_entry,
            commands::sync_queue::count_sync_queue_entries,
            commands::sync_queue::remove_sync_queue_entry,
            commands::sync_queue::run_sync,
            commands::todo::change_todo_priority,
            commands::todo::create_todo,
            commands::todo::delete_todo,
            commands::todo::duplicate_todo,
            commands::todo::get_all_todos,
            commands::todo::get_todo,
            commands::todo::mark_todo_done,
            commands::todo::transfer_todo,
            commands::todo::update_todo,
            commands::todo::update_todo_due_date,
            commands::user_preference::create_user_preference,
            commands::user_preference::duplicate_user_preference,
            commands::user_preference::get_user_preference,
            commands::user_preference::transfer_user_preference,
            commands::user_preference::update_user_preference,
            commands::workspaces::create_workspace,
            commands::workspaces::delete_workspace,
            commands::workspaces::get_workspace_by_id,
            commands::workspaces::list_workspaces,
            commands::workspaces::update_workspace,
            commands::workspaces::verify_workspace_password,
            commands::moodboard::save_moodboard_image,
            commands::moodboard::list_moodboard_images,
            commands::moodboard::delete_moodboard_image,
            commands::ollama::is_ollama_installed,
            commands::ollama::check_ai_model,
            commands::ollama::pull_ai_model,
            commands::ollama::generate_stream,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
