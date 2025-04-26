use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MaterialeInfissoDTO {
    pub materiale: String,
    pub efficienza_energetica: u8,
}
