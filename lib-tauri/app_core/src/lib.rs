use crate::constants::NAME_DIR_DATABASE;
use app_database::database::DatabaseManager;
use app_services::service::EdificioSelected;
use dirs_next::document_dir;
use std::sync::Arc;
use tauri::async_runtime::RwLock;
use tauri::path::BaseDirectory;
use tauri::{App, AppHandle, Builder, Manager, Wry};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};

// mod dao;
mod constants;

pub fn initialize_tauri() -> Builder<Wry> {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main windows")
                .set_focus();
        }))
        .plugin(tauri_plugin_dialog::init())
        .setup(|app: &mut App| {
            setup_logger(app)?;

            // Manage Database
            tauri::async_runtime::block_on(async {
                let database = DatabaseManager::new().await;
                app.manage(database);
            });

            // Manage Edificio Selected
            let stato_edificio = Arc::new(RwLock::new(EdificioSelected::new()));
            app.manage(stato_edificio);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            /*
                // miscellaneous
                export_data_to_excel,
                init_to_excel,
                // database
                set_database,
                switch_database,
                close_database,
                get_all_name_database,
                // tipi
                get_all_tipi,
                insert_tipo,
                // stanza
                get_stanze,
                insert_stanza,
                update_stanza,
                // infisso
                get_infissi,
                insert_infisso,
                update_infisso,
                // edificio
                get_edifici,
                update_edificio,
                // utenze
                get_utenze,
                insert_utenza,
                // fotovoltaico
                get_fotovoltaico,
                insert_fotovoltaico,
                // annotazioni
                get_annotazioni,
                insert_annotazione
            */
        ])
        .on_window_event(handle_window_events)
}

fn handle_window_events(windows: &tauri::Window, event: &tauri::WindowEvent) {
    if let tauri::WindowEvent::CloseRequested { .. } = event {
        let db = windows.app_handle().state::<DatabaseManager>();
        // fixme: eseguire qualcosa alla chiusura del database
        // match close_database(db) {
        //     Ok(..) => info!("Database chiuso correttamente"),
        //     Err(e) => error!("Errore durante la chiusura del database: {}", e),
        // }
        clear_app_data(windows.app_handle()).unwrap_or_default();
    }
}

fn setup_logger(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let mut log_directory = document_dir().unwrap();
    log_directory.push(format!("{NAME_DIR_DATABASE}/log"));
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
            .max_file_size(50000 /* 50kb */)
            .build(),
    )?;
    Ok(())
}

/// Rimuove la directory dei dati del frontend dell'applicazione.
fn clear_app_data(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    match app.path().resolve("", BaseDirectory::AppData) {
        Ok(path) => {
            std::fs::remove_dir_all(path)?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
