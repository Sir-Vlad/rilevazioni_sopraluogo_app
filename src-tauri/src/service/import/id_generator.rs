use crate::dao::entity::Stanza;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum TipologieStanze {
    Magazzino,
    Corridoio,
    Archivio,
    VanoTecnico,
    SpazioServizi,
    Spogliatoi,
    Bagno,
    Ripostiglio,
    Atrio,
    VanoScale,
    Garage,
    Disimpegno,
    Ufficio,
    SalaConvegli,
    Palestra,
    Altro,
}

impl Display for TipologieStanze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TipologieStanze::Magazzino => f.write_str("Magazzino"),
            TipologieStanze::Corridoio => f.write_str("Corridoio"),
            TipologieStanze::Archivio => f.write_str("Archivio"),
            TipologieStanze::VanoTecnico => f.write_str("Vano tecnico"),
            TipologieStanze::SpazioServizi => f.write_str("Spazio servizi"),
            TipologieStanze::Spogliatoi => f.write_str("Spogliatoi"),
            TipologieStanze::Bagno => f.write_str("Bagno"),
            TipologieStanze::Ripostiglio => f.write_str("Ripostiglio"),
            TipologieStanze::Atrio => f.write_str("Atrio"),
            TipologieStanze::VanoScale => f.write_str("Vano scale"),
            TipologieStanze::Garage => f.write_str("Garage"),
            TipologieStanze::Disimpegno => f.write_str("Disimpegno"),
            TipologieStanze::Ufficio => f.write_str("Ufficio"),
            TipologieStanze::SalaConvegli => f.write_str("Sala convegli"),
            TipologieStanze::Palestra => f.write_str("Palestra"),
            TipologieStanze::Altro => f.write_str("Altro"),
        }
    }
}

impl PartialEq<&str> for TipologieStanze {
    fn eq(&self, other: &&str) -> bool {
        self.to_string().eq_ignore_ascii_case(other)
    }
}

impl From<&str> for TipologieStanze {
    fn from(s: &str) -> Self {
        match s {
            "Magazzino" => TipologieStanze::Magazzino,
            "Corridoio" => TipologieStanze::Corridoio,
            "Archivio" => TipologieStanze::Archivio,
            "Vano tecnico" | "Vano Tecnico" => TipologieStanze::VanoTecnico,
            "Spazio servizi" | "Spazio Servizi" => TipologieStanze::SpazioServizi,
            "Spogliatoi" => TipologieStanze::Spogliatoi,
            "Bagno" => TipologieStanze::Bagno,
            "Ripostiglio" => TipologieStanze::Ripostiglio,
            "Atrio" => TipologieStanze::Atrio,
            "Vano scale" | "Vano Scale" => TipologieStanze::VanoScale,
            "Garage" => TipologieStanze::Garage,
            "Disimpegno" => TipologieStanze::Disimpegno,
            "Ufficio" => TipologieStanze::Ufficio,
            "Sala convegli" | "Sala Convegli" => TipologieStanze::SalaConvegli,
            "Palestra" => TipologieStanze::Palestra,
            _ => TipologieStanze::Altro,
        }
    }
}

pub struct IdGeneratorStanza {
    counters: HashMap<String, HashMap<TipologieStanze, u32>>,
}

impl IdGeneratorStanza {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    pub fn generate_id(&mut self, mut stanza: Stanza) -> Stanza {
        if stanza.cod_stanza == "_" {
            let tipo = TipologieStanze::from(stanza.destinazione_uso.as_str());
            self.counters
                .entry(stanza.piano.clone())
                .or_insert_with(HashMap::new)
                .entry(tipo.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);

            let cod_tipo = match tipo {
                TipologieStanze::Magazzino => "MAG",
                TipologieStanze::Corridoio => "COR",
                TipologieStanze::Archivio => "ARC",
                TipologieStanze::VanoTecnico => "TEC",
                TipologieStanze::SpazioServizi => "SER",
                TipologieStanze::Spogliatoi => "SPO",
                TipologieStanze::Bagno => "BGN",
                TipologieStanze::Ripostiglio => "RPS",
                TipologieStanze::Atrio => "ATR",
                TipologieStanze::VanoScale => "SCA",
                TipologieStanze::Garage => "GRG",
                TipologieStanze::Disimpegno => "DSM",
                TipologieStanze::Ufficio => "UFF",
                TipologieStanze::SalaConvegli => "CVG",
                TipologieStanze::Palestra => "PLS",
                TipologieStanze::Altro => "AA",
            };

            stanza.cod_stanza = format!(
                "{}_{}_{:02}",
                stanza.piano, cod_tipo, self.counters[&stanza.piano][&tipo]
            );

            return stanza;
        }
        stanza
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let mut id_generator = IdGeneratorStanza::new();

        let stanza = id_generator.generate_id(Stanza {
            id: None,
            chiave: "".to_string(),
            piano: "-1".to_string(),
            id_spazio: "".to_string(),
            cod_stanza: "_".to_string(),
            destinazione_uso: "Archivio".to_string(),
            altezza: None,
            spessore_muro: None,
            riscaldamento: None,
            raffrescamento: None,
            illuminazione: None,
        });

        let stanza = id_generator.generate_id(Stanza {
            id: None,
            chiave: "".to_string(),
            piano: "-1".to_string(),
            id_spazio: "".to_string(),
            cod_stanza: "_".to_string(),
            destinazione_uso: "Archivio".to_string(),
            altezza: None,
            spessore_muro: None,
            riscaldamento: None,
            raffrescamento: None,
            illuminazione: None,
        });

        assert_eq!(stanza.cod_stanza, "-1_ARC_02");
    }
}
