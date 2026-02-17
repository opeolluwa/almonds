#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}))
        // .plugin(tauri_plugin_sql::Builder::new().build())
        // .setup(|app| {
        //     if cfg!(debug_assertions) {
        //         app.handle().plugin(
        //             tauri_plugin_log::Builder::default()
        //                 .level(log::LevelFilter::Info)
        //                 .build(),
        //         )?;
        //     }
        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
