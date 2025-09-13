use app_state::selected_edificio::StateEdificioSelected;
use std::ops::Deref;
use tauri::State;

pub mod command;
mod events_payload;

async fn is_selected_edificio(selected_edificio: State<'_, StateEdificioSelected>) -> bool {
    get_chiave_selected_edificio(selected_edificio).await.is_some()
}

async fn get_chiave_selected_edificio(
    selected_edificio: State<'_, StateEdificioSelected>,
) -> Option<String> {
    selected_edificio.deref().read().await.deref().get_chiave()
}
