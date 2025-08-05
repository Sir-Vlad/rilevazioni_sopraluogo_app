use std::env;
use std::path::Path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::from_path(Path::new("./src-tauri/.env")).ok();

    app_core::initialize_tauri()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
