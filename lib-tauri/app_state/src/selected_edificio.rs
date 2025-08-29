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

impl EdificioSelected {
    pub fn new() -> Self {
        Self { chiave: None }
    }

    pub fn set_chiave(&mut self, chiave: String) {
        self.chiave = Some(chiave);
    }

    pub fn get_chiave(&self) -> Option<String> {
        self.chiave.clone()
    }

    pub fn clear_edificio(&mut self) {
        self.chiave = None;
    }
}

pub type StateEdificioSelected = Arc<RwLock<EdificioSelected>>;