use crate::app_traits::{EntityTrait, FromRow, SqlParams, ToInsert, ToRetrieveAll, ToUpdate};
use crate::database::QueryBuilderError;
use crate::dto::EdificioDTO;
use crate::utils::ToList;
use rusqlite::{Error, Row};
use std::any::Any;

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct Edificio {
    pub(crate) chiave: String,
    pub(crate) fascicolo: String,
    pub(crate) indirizzo: String,
    pub(crate) anno_costruzione: Option<String>,
    pub(crate) anno_riqualificazione: Option<String>,
    pub(crate) note_riqualificazione: Option<String>,
    pub(crate) isolamento_tetto: Option<bool>,
    pub(crate) cappotto: Option<bool>,
}

impl Edificio {
    pub fn new(chiave: &str, fascicolo: &str, indirizzo: &str) -> Self {
        Edificio {
            chiave: chiave.to_string(),
            fascicolo: fascicolo.to_string(),
            indirizzo: indirizzo.to_string(),
            anno_costruzione: None,
            anno_riqualificazione: None,
            note_riqualificazione: None,
            isolamento_tetto: None,
            cappotto: None,
        }
    }

    pub fn get_fields() -> Vec<String> {
        vec![
            "chiave".to_string(),
            "fascicolo".to_string(),
            "indirizzo".to_string(),
            "anno_costruzione".to_string(),
            "anno_riqualificazione".to_string(),
            "note_riqualificazione".to_string(),
            "isolamento_tetto".to_string(),
            "cappotto".to_string(),
        ]
    }
}

impl From<EdificioDTO> for Edificio {
    fn from(value: EdificioDTO) -> Self {
        Edificio {
            chiave: value.chiave.to_string(),
            fascicolo: value.fascicolo.to_string(),
            indirizzo: value.indirizzo.to_string(),
            anno_costruzione: value.anno_costruzione.clone(),
            anno_riqualificazione: value.anno_riqualificazione.clone(),
            note_riqualificazione: None,
            isolamento_tetto: value.isolamento_tetto,
            cappotto: value.cappotto,
        }
    }
}

impl ToList for Edificio {
    fn to_list(&self) -> Vec<Box<dyn Any>> {
        vec![
            Box::new(self.chiave.clone()),
            Box::new(self.fascicolo.clone()),
            Box::new(self.indirizzo.clone()),
            Box::new(self.anno_costruzione.clone()),
            Box::new(self.anno_riqualificazione.clone()),
            Box::new(self.note_riqualificazione.clone()),
            Box::new(self.isolamento_tetto),
            Box::new(self.cappotto),
        ]
    }
}

impl FromRow for Edificio {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            chiave: row.get("CHIAVE")?,
            fascicolo: row.get("FASCICOLO")?,
            indirizzo: row.get("INDIRIZZO")?,
            anno_costruzione: row.get("ANNO_COSTRUZIONE")?,
            anno_riqualificazione: row.get("ANNO_RIQUALIFICAZIONE")?,
            note_riqualificazione: row.get("NOTE_RIQUALIFICAZIONE")?,
            isolamento_tetto: row.get("ISOLAMENTO_TETTO")?,
            cappotto: row.get("CAPPOTTO")?,
        })
    }
}

impl EntityTrait for Edificio {
    type PrimaryKey = String;

    #[inline]
    fn table_name() -> String {
        "EDIFICIO".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    CHIAVE                TEXT PRIMARY KEY,
                    FASCICOLO             TEXT NOT NULL,
                    INDIRIZZO             TEXT NOT NULL,
                    ANNO_COSTRUZIONE      TEXT    DEFAULT NULL,
                    ANNO_RIQUALIFICAZIONE TEXT    DEFAULT NULL,
                    NOTE_RIQUALIFICAZIONE TEXT    DEFAULT NULL,
                    ISOLAMENTO_TETTO      INTEGER DEFAULT FALSE,
                    CAPPOTTO              INTEGER DEFAULT FALSE
                ) STRICT;",
            Self::table_name()
        )
    }
}

impl ToRetrieveAll for Edificio {}

impl ToInsert for Edificio {
    fn to_insert() -> String {
        "INSERT INTO EDIFICIO (CHIAVE, FASCICOLO, INDIRIZZO)  VALUES (?, ?, ?) RETURNING *"
            .to_string()
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.chiave, &self.fascicolo, &self.indirizzo]
    }
}

impl ToUpdate for Edificio {
    fn to_update() -> String {
        panic!("usare to_build_update()")
    }

    fn to_build_update(
        &self,
    ) -> Result<Option<(String, Vec<Box<&dyn SqlParams>>)>, QueryBuilderError> {
        let mut set_clauses = Vec::new();
        let mut params: Vec<Box<&dyn SqlParams>> = Vec::new();

        // Aggiungere clausole dinamiche
        if let Some(ref anno_costruzione) = self.anno_costruzione {
            set_clauses.push("ANNO_COSTRUZIONE = ?");
            params.push(Box::new(anno_costruzione));
        }
        if let Some(ref anno_riqualificazione) = self.anno_riqualificazione {
            set_clauses.push("ANNO_RIQUALIFICAZIONE = ?");
            params.push(Box::new(anno_riqualificazione));
        }
        if let Some(ref note_riqualificazione) = self.note_riqualificazione {
            set_clauses.push("NOTE_RIQUALIFICAZIONE = ?");
            params.push(Box::new(note_riqualificazione));
        }
        if let Some(ref isolamento_tetto) = self.isolamento_tetto {
            set_clauses.push("ISOLAMENTO_TETTO = ?");
            params.push(Box::new(isolamento_tetto));
        }
        if let Some(ref cappotto) = self.cappotto {
            set_clauses.push("CAPPOTTO = ?");
            params.push(Box::new(cappotto));
        }

        // Se non ci sono campi da aggiornare, ritorniamo None
        if set_clauses.is_empty() {
            return Ok(None);
        }

        // Costruzione condizione WHERE
        let mut query = format!(
            "UPDATE {} SET {}",
            Self::table_name(),
            set_clauses.join(", ")
        );
        query.push_str(" WHERE CHIAVE = ?");
        params.push(Box::new(&self.chiave));

        query = format!("{} RETURNING *", query);

        Ok(Some((query, params)))
    }

    fn to_update_params(&self) -> Vec<Box<&dyn SqlParams>> {
        panic!("usare to_build_update()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_build_update() {
        let mut entity = Edificio {
            chiave: "1234567890".to_string(),
            fascicolo: "1234567890".to_string(),
            indirizzo: "1234567890".to_string(),
            anno_costruzione: Some("2020".to_string()),
            anno_riqualificazione: Some("2021".to_string()),
            note_riqualificazione: Some("Test".to_string()),
            isolamento_tetto: Some(true),
            cappotto: Some(true),
        };
        let res = entity.to_build_update().unwrap().unwrap();
        println!("{:?}", res.0);

        entity.cappotto = None;
        entity.note_riqualificazione = None;

        let res = entity.to_build_update().unwrap().unwrap();
        println!("{:?}", res.0);
        assert_eq!(
            res.0,
            "UPDATE EDIFICIO SET ANNO_COSTRUZIONE = ?, ANNO_RIQUALIFICAZIONE = ?, ISOLAMENTO_TETTO = ? WHERE CHIAVE = ?"
        );
        assert_eq!(res.1.len(), 4);
    }
}
