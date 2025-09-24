use app_models::models::Stanza;
use app_utils::app_error::{AppResult, DomainError, ErrorKind};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidPiano,
    InvalidUso,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidPiano => f.write_str("Piano non valido o vuoto"),
            Error::InvalidUso => f.write_str("Destinazione uso non valido o vuoto"),
        }
    }
}

impl std::error::Error for Error {}

type IdEdificio = String;
type Piano = String;
type DestUso = String;

pub struct IdGeneratorStanza {
    /// (chiave, piano) -> (destinazione uso) -> contatore
    counters: HashMap<(IdEdificio, Piano), HashMap<DestUso, u32>>,
}

impl Default for IdGeneratorStanza {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGeneratorStanza {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    pub fn generate_id(&mut self, mut stanza: Stanza) -> AppResult<Stanza> {
        let piano = if let Some(piano) = self.format_piano(stanza.piano.as_str()) {
            piano
        } else {
            return Err(DomainError::InvalidInput(ErrorKind::FormatInvalid, "Piano non valido o vuoto".to_string()).into());
        };

        let des_uso = if let Some(des_uso) = self.format_uso(stanza.destinazione_uso.as_str()) {
            des_uso
        } else {
            return Err(DomainError::InvalidInput(ErrorKind::FormatInvalid, "Destinazione d'uso non valido o vuoto".to_string()).into());
        };

        let key_hash = (stanza.edificio_id.clone(), piano.clone());

        // Increment the counter for the given key
        self.counters
            .entry(key_hash.clone())
            .or_default()
            .entry(des_uso.clone())
            .and_modify(|e| *e += 1)
            .or_insert(1);

        stanza.cod_stanza = format!(
            "{}_{}_{:02}",
            piano, des_uso, self.counters[&key_hash][&des_uso]
        );

        Ok(stanza)
    }

    fn format_piano(&self, piano: &str) -> Option<String> {
        match piano.trim() {
            s if s.parse::<i32>().is_ok_and(|i| i < 0) => {
                Some(format!("S{:02}", s.parse::<i32>().unwrap().abs()))
            }
            "T" => Some("PT".to_string()),
            s if s.parse::<i32>().is_ok_and(|i| i > 0) => {
                Some(format!("P{:02}", s.parse::<i32>().unwrap()))
            }
            _ => None,
        }
    }

    fn format_uso(&self, uso: &str) -> Option<String> {
        if uso.is_empty() {
            return None;
        }

        let split: Vec<_> = uso
            .trim()
            .split(" ")
            .map(|w| w.to_ascii_uppercase())
            .collect();
        if split.len() == 1 {
            return Some(split[0].chars().take(3).collect());
        }

        // Get the first letter of each word
        let mut v: Vec<char> = split.iter().filter_map(|s| s.chars().next()).collect();
        // If the first word is longer than 3 chars, add the remaining chars to the first word
        if v.len() < 3 {
            if let Some(last_word) = split.last() {
                let remaining_chars = last_word.chars().skip(1);
                for c in remaining_chars {
                    v.push(c);
                    if v.len() >= 3 {
                        break;
                    }
                }
            }
        }
        // Truncate the vector to 3 chars
        v.truncate(3);
        Some(v.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use app_models::models::Stanza;
    use app_utils::app_error::ApplicationError;

    fn init_stanza(piano: Piano, cod_stanza: String, dest_uso: DestUso) -> Stanza {
        Stanza {
            id: 0,
            edificio_id: "1234567890".to_string(),
            piano,
            id_spazio: "".to_string(),
            cod_stanza,
            destinazione_uso: dest_uso,
            altezza: None,
            spessore_muro: None,
            riscaldamento: None,
            raffrescamento: None,
            illuminazione: None,
        }
    }

    #[test]
    fn test_piano_interrato() {
        let id_gen = IdGeneratorStanza::new();
        let res = id_gen.format_piano("-2").unwrap();
        assert_eq!(res, "S02");
    }

    #[test]
    fn test_piano_terra() {
        let id_gen = IdGeneratorStanza::new();
        let res = id_gen.format_piano("T").unwrap();
        assert_eq!(res, "PT");
    }

    #[test]
    fn test_piano_sopraelevato() {
        let id_gen = IdGeneratorStanza::new();
        let res = id_gen.format_piano("2").unwrap();
        assert_eq!(res, "P02");
    }

    #[test]
    fn test_uso_empty() {
        let id_gen = IdGeneratorStanza::new();
        let res = id_gen.format_uso("");
        assert!(res.is_none());
    }

    #[test]
    fn test_uso_singola_parola() {
        let id_gen = IdGeneratorStanza::new();
        let res = id_gen.format_uso("Ufficio").unwrap();
        assert_eq!(res, "UFF");
    }

    #[test]
    fn test_uso_multipla_parola() {
        let id_gen = IdGeneratorStanza::new();
        let res = id_gen.format_uso("Vano Scala").unwrap();
        assert_eq!(res, "VSC");
    }

    #[test]
    fn test_generate_id_singola_parola_piano_terra() {
        let mut id_gen = IdGeneratorStanza::new();
        let stanza = init_stanza("T".to_string(), "_".to_string(), "Ufficio".to_string());

        let id = id_gen.generate_id(stanza).unwrap();
        assert_eq!(id.cod_stanza, "PT_UFF_01");
    }

    #[test]
    fn test_generate_id_singola_parola_piano_interrato() {
        let mut id_gen = IdGeneratorStanza::new();
        let stanza = init_stanza("-3".to_string(), "_".to_string(), "Ufficio".to_string());

        let id = id_gen.generate_id(stanza).unwrap();
        assert_eq!(id.cod_stanza, "S03_UFF_01");
    }

    #[test]
    fn test_generate_id_singola_parola_piano_sopraelevato() {
        let mut id_gen = IdGeneratorStanza::new();
        let stanza = init_stanza("3".to_string(), "_".to_string(), "Ufficio".to_string());

        let id = id_gen.generate_id(stanza).unwrap();
        assert_eq!(id.cod_stanza, "P03_UFF_01");
    }

    #[test]
    fn test_generate_id_multipla_parola() {
        let mut id_gen = IdGeneratorStanza::new();
        let stanza = init_stanza("3".to_string(), "_".to_string(), "Sala Convegli".to_string());

        let id = id_gen.generate_id(stanza).unwrap();
        assert_eq!(id.cod_stanza, "P03_SCO_01");
    }

    #[test]
    fn test_generate_id_multipli() {
        let mut id_gen = IdGeneratorStanza::new();
        let stanza = init_stanza("3".to_string(), "_".to_string(), "Sala Convegli".to_string());
        let _ = id_gen.generate_id(stanza);

        let stanza = init_stanza("3".to_string(), "_".to_string(), "Sala Convegli".to_string());
        let id = id_gen.generate_id(stanza).unwrap();
        assert_eq!(id.cod_stanza, "P03_SCO_02");
    }

    #[test]
    fn test_generate_id_error_piano() {
        let mut id_gen = IdGeneratorStanza::new();
        let stanza = init_stanza("".to_string(), "_".to_string(), "Sala Convegli".to_string());

        let id = id_gen.generate_id(stanza);
        match id {
            Ok(_) => {}
            Err(ApplicationError::Domain(DomainError::InvalidInput(error_kind, msg))) => {
                assert_eq!(error_kind, ErrorKind::FormatInvalid);
            }
            _ => {}
        }
    }

    #[test]
    fn test_generate_id_error_uso() {
        let mut id_gen = IdGeneratorStanza::new();
        let stanza = init_stanza("2".to_string(), "_".to_string(), "".to_string());

        let id = id_gen.generate_id(stanza);
        match id {
            Ok(_) => {}
            Err(ApplicationError::Domain(DomainError::InvalidInput(error_kind, msg))) => {
                assert_eq!(error_kind, ErrorKind::FormatInvalid);
            }
            _ => {}
        }
    }
}
