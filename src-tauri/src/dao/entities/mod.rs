mod climatizzazione;
mod edificio;
mod illuminazione;
mod infisso;
mod materiale_infisso;
mod stanza;
mod vetro_infisso;

pub mod entity {
    pub use super::climatizzazione::Climatizzazione;
    pub use super::edificio::Edificio;
    pub use super::illuminazione::Illuminazione;
    pub use super::infisso::Infisso;
    pub use super::materiale_infisso::MaterialeInfisso;
    pub use super::stanza::Stanza;
    pub use super::vetro_infisso::VetroInfisso;
}
