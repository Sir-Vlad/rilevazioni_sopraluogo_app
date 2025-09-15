pub use app_utils::app_interface::service_interface::{SelectedEdificioState, SelectedEdificioTrait};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct EdificioSelected {
    chiave: Option<String>,
}

impl Default for EdificioSelected {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectedEdificioTrait for EdificioSelected {
    fn new() -> Self {
        Self { chiave: None }
    }
    
    fn set_chiave(&mut self, chiave: String) {
        self.chiave = Some(chiave);
    }

    fn get_chiave(&self) -> Option<String> {
        self.chiave.clone()
    }

    fn clear_chiave(&mut self) {
        self.chiave = None;
    }
}