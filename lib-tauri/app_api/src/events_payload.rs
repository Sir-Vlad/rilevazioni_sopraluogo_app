use app_services::dto::EdificioDTO;
use serde::Serialize;

#[derive(Serialize, Clone)]
#[non_exhaustive]
pub enum TypeEvent {
    ChangedEdificio,
    NewEdificio,
}

#[derive(Serialize, Clone)]
pub struct EventWrapper<T>
where
    T: Serialize + Clone,
{
    type_event: TypeEvent,
    timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    payload: T,
}

impl<T> EventWrapper<T>
where
    T: Serialize + Clone,
{
    pub fn new(type_event: TypeEvent, payload: T) -> Self {
        Self {
            type_event,
            timestamp: chrono::Utc::now(),
            payload,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct EdificioChangePayload {
    chiave: String,
}

impl EdificioChangePayload {
    pub fn new(chiave: String) -> Self { Self { chiave } }
}

#[derive(Serialize, Clone)]
pub struct NewEdificioPayload {
    edifici: Vec<EdificioDTO>,
    edificio_selected: String,
}

impl NewEdificioPayload {
    pub fn new(edifici: Vec<EdificioDTO>, edificio_selected: String) -> Self {
        Self {
            edifici,
            edificio_selected,
        }
    }
}
