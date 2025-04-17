mod command;
mod dao;
mod database;
mod dto;
mod service;

use crate::command::command_tauri::*;
use crate::database::*;
use database::NAME_DIR_DATABASE;
use dirs_next::document_dir;
use tauri::{App, Manager};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main windows")
                .set_focus();
        }))
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Database::default());
            setup_logger(app)?;
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
            export_data_to_excel,
            get_edifici,
            update_edificio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_logger(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let mut log_directory = document_dir().unwrap();
    log_directory.push(format!("{}/log", NAME_DIR_DATABASE));
    app.handle().plugin(
        tauri_plugin_log::Builder::new()
            .target(Target::new(TargetKind::Stdout))
            .target(Target::new(TargetKind::Folder {
                path: log_directory,
                file_name: None,
            }))
            .target(Target::new(TargetKind::Webview))
            .level(log::LevelFilter::Info)
            .rotation_strategy(RotationStrategy::KeepAll)
            .max_file_size(50000)
            .build(),
    )?;
    Ok(())
}
