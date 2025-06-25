use crate::db::DatabaseEventPayload;
use tauri::{AppHandle, Emitter, Error};

pub fn database_change_event(app_handle: AppHandle, db_path: String) -> Result<(), Error> {
    app_handle.emit(
        "db-changed",
        DatabaseEventPayload {
            type_event: "database_switched",
            path: db_path,
        },
    )
}
