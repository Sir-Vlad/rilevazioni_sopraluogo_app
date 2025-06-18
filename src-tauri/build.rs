fn main() {
    let windows = tauri_build::WindowsAttributes::new();
    let manifest = include_str!("app.manifest");

    let windows = windows.app_manifest(manifest);
    let attrs = tauri_build::Attributes::new().windows_attributes(windows);

    tauri_build::try_build(attrs).expect("Failed to build tauri application")
}
