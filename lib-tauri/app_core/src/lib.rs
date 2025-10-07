use std::{
    error,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use app_api::command::*;
use app_state::{
    database::DatabaseManager,
    selected_edificio::{EdificioSelected, SelectedEdificioTrait},
};
use app_task_background::get_background_manager;
use dirs_next::document_dir;
use log::{error, info, warn};
use tauri::{App, Builder, Manager, Wry, async_runtime::RwLock};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};
use tauri_plugin_notification::NotificationExt;
use tokio::time::timeout;

use crate::constants::NAME_DIR_DATABASE;

mod constants;

static SHUTDOWN_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

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
        .plugin(tauri_plugin_notification::init())
        // note: the path where the database was saved is ~/.local/share/<id-badle>
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app: &mut App| {
            setup_logger(app)?;
            info!("Starting application ...");

            // Manage Database
            tauri::async_runtime::block_on(async {
                let database = DatabaseManager::new().await;
                info!("Database Manager is created successfully");
                app.manage(database);
            });

            // Manage Edificio Selected
            let stato_edificio = Arc::new(RwLock::new(EdificioSelected::new()));
            app.manage(stato_edificio);

            // Starting the task in background
            let bg_manager = get_background_manager();
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let mut manager = bg_manager.lock().await;
                let app_handle_arc = Arc::new(app_handle);

                if let Err(e) = manager.start(app_handle_arc).await {
                    eprintln!("Errore during starting the Background Manager: {}", e);
                }
            });

            info!("Started application!!!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // miscellaneous
            //export_data_to_excel,
            add_new_fascicolo_from_xlsx,
            // database
            set_edificio,
            clear_edificio,
            get_fascicoli,
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
        ])
        .on_window_event(handle_window_events)
}

fn handle_window_events(windows: &tauri::Window, event: &tauri::WindowEvent) {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        if SHUTDOWN_IN_PROGRESS
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }

        api.prevent_close();

        let windows_clone = windows.clone();
        let bg_manager = get_background_manager();

        tauri::async_runtime::spawn(async move {
            if let Err(e) = windows_clone
                .app_handle()
                .notification()
                .builder()
                .title("Chiusura applicazione")
                .body("Sto fermando i processi in background ...")
                .show()
            {
                error!("Don't show notification: {}", e)
            }

            let shutdown_start = std::time::Instant::now();
            let shutdown_result = timeout(Duration::from_secs(10), async {
                // Closed task in background
                bg_manager.lock().await.stop().await;
            })
            .await;

            let elapsed = shutdown_start.elapsed();
            match shutdown_result {
                Ok(_) => {
                    info!(
                        "Shutdown complete successfully in {}ms",
                        elapsed.as_millis()
                    );
                    if let Err(e) = windows_clone
                        .app_handle()
                        .notification()
                        .builder()
                        .title("Chiusura completata")
                        .body("Tutti i processi sono stati fermati correttamente")
                        .show()
                    {
                        error!("Errore notification: {}", e);
                    }
                }
                Err(_) => {
                    warn!("⚠️ Timeout in shutdown after {:?} - close forced", elapsed);

                    // Notifica di timeout
                    if let Err(e) = windows_clone
                        .app_handle()
                        .notification()
                        .builder()
                        .title("Avviso chiusura")
                        .body("Timeout raggiunto, chiusura forzata dei processi")
                        .show()
                    {
                        error!("Errore notification: {}", e);
                    }
                }
            }

            windows_clone.close().unwrap();
        });
    }
}

fn setup_logger(app: &mut App) -> Result<(), Box<dyn error::Error>> {
    let mut log_directory = document_dir().unwrap();
    log_directory.push(format!("{NAME_DIR_DATABASE}/log"));

    let level_filter: log::LevelFilter = std::env::var("RUST_LOG")
        .ok()
        .map(|s| s.parse().ok().unwrap())
        .unwrap_or(log::LevelFilter::Info);

    app.handle().plugin(
        tauri_plugin_log::Builder::new()
            .target(Target::new(TargetKind::Webview))
            .level(level_filter)
            .rotation_strategy(RotationStrategy::KeepAll)
            .max_file_size(50000 /* 50kb */)
            .build(),
    )?;
    Ok(())
}
