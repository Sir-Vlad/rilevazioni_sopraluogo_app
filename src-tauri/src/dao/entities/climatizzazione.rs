#[derive(Debug, PartialEq)]
pub struct Climatizzazione {
    #[allow(dead_code)]
    pub id: u64,
    pub climatizzazione: String,
    pub efficienza_energetica: u8,
}
