mod database;

use crate::database::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main windows")
                .set_focus();
        }))
        .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_devtools::init())
        .setup(|app| {
            app.manage(Database::default());
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
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
            insert_stanze,
            update_stanza,
            insert_stanza_con_infissi,
            get_stanza_con_infissi
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
