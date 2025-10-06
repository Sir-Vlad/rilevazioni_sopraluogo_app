#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    app_core::initialize_tauri()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
