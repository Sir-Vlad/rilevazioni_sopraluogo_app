mod database;

use database::{elaborate_file, get_all_name_database, get_db_path};
use tauri_plugin_sql::Builder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(Builder::default().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_db_path, get_all_name_database, elaborate_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
