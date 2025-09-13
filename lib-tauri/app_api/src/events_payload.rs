use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct EdificioChangeEventPayload {
    pub type_event: &'static str,
    pub chiave: String,
}