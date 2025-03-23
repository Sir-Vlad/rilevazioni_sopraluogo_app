mod database;

use crate::database::{
    get_all_name_database, get_infissi, get_stanze, get_types, insert_infisso, insert_stanze,
    set_database, switch_database, Database,
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Database::default());
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_database,
            switch_database,
            get_all_name_database,
            get_stanze,
            get_infissi,
            get_types,
            insert_infisso,
            insert_stanze
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
