use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VetroInfissoDTO {
    pub vetro: String,
    pub efficienza_energetica: u8,
}
