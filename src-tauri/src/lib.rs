mod command;
mod dao;
mod database;
mod dto;
mod service;

use crate::command::command_tauri::{
    export_data_to_excel, get_all_tipi, get_climatizzazione, get_illuminazione, get_infissi,
    get_infissi_stanza, get_materiali_infisso, get_stanze, get_vetro_infisso, init_to_excel,
    insert_infissi_stanza, insert_infisso, insert_stanza, update_infisso, update_stanza,
};
use crate::database::*;
use tauri::{App, Manager};

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
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init_to_excel,
            set_database,
            switch_database,
            get_all_name_database,
            get_all_tipi,
            get_materiali_infisso,
            get_vetro_infisso,
            get_illuminazione,
            get_climatizzazione,
            get_stanze,
            insert_stanza,
            update_stanza,
            get_infissi_stanza,
            insert_infissi_stanza,
            get_infissi,
            insert_infisso,
            update_infisso,
            export_data_to_excel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
