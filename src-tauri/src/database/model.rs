use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

pub struct Database {
    conn: Mutex<Option<Connection>>,
    path_to_database: Mutex<Option<String>>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            conn: Mutex::new(None),
            path_to_database: Mutex::new(None),
        }
    }
}

impl Database {
    pub fn get_path_to_database(&self) -> MutexGuard<'_, Option<String>> {
        self.path_to_database.lock().unwrap()
    }
    
    pub fn get_conn(&self) -> MutexGuard<'_, Option<Connection>> {
        self.conn.lock().unwrap()
    }

    pub fn with_transaction<F, T>(&self, op: F) -> Result<T, String>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
        F: FnOnce(&rusqlite::Transaction) -> Result<T, String>,
    {
        let mut conn_guard = self.conn.lock().unwrap();
        if let Some(conn) = conn_guard.as_mut() {
            let tx = conn.transaction().map_err(|e| e.to_string())?;
            let result = op(&tx)?;
            tx.commit().map_err(|e| e.to_string())?;
            Ok(result)
        } else {
            Err("Database not initialized".to_string())
        }
    }
}

#[derive(Serialize, Clone)]
pub struct DatabaseEventPayload {
    pub(crate) type_event: &'static str,
    pub(crate) path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Infisso {
    pub id: String,
    pub tipo: String,
    pub altezza: i32,
    pub larghezza: i32,
    pub materiale: String,
    pub vetro: String,
}

impl Infisso {
    pub fn new(
        id: String,
        tipo: String,
        altezza: i32,
        larghezza: i32,
        materiale: String,
        vetro: String,
    ) -> Self {
        Self {
            id,
            tipo,
            altezza,
            larghezza,
            materiale,
            vetro,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stanza {
    pub id: u64,
    pub chiave: String,
    pub piano: String,
    pub id_spazio: String,
    pub stanza: String,
    pub destinazione_uso: String,
    pub altezza: Option<u16>,
    pub spessore_muro: Option<u8>,
    pub riscaldamento: Option<String>,
    pub raffrescamento: Option<String>,
    pub illuminazione: Option<String>,
}

pub(crate) struct StanzaBuilder {
    pub id: Option<u64>,
    pub chiave: Option<String>,
    pub piano: Option<String>,
    pub id_spazio: Option<String>,
    pub stanza: Option<String>,
    pub destinazione_uso: Option<String>,
    pub altezza: Option<u16>,
    pub spessore_muro: Option<u8>,
    pub riscaldamento: Option<String>,
    pub raffrescamento: Option<String>,
    pub illuminazione: Option<String>,
}

impl StanzaBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            chiave: None,
            piano: None,
            id_spazio: None,
            stanza: None,
            destinazione_uso: None,
            altezza: None,
            spessore_muro: None,
            riscaldamento: None,
            raffrescamento: None,
            illuminazione: None,
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn chiave(mut self, chiave: String) -> Self {
        self.chiave = Some(chiave);
        self
    }

    pub fn piano(mut self, piano: String) -> Self {
        self.piano = Some(piano);
        self
    }

    pub fn id_spazio(mut self, id_spazio: String) -> Self {
        self.id_spazio = Some(id_spazio);
        self
    }

    pub fn stanza(mut self, stanza: String) -> Self {
        self.stanza = Some(stanza);
        self
    }

    pub fn destinazione_uso(mut self, destinazione_uso: String) -> Self {
        self.destinazione_uso = Some(destinazione_uso);
        self
    }

    pub fn altezza(mut self, altezza: Option<u16>) -> Self {
        if altezza.is_none() {
            self.altezza = None;
        } else {
            self.altezza = Some(altezza.unwrap());
        }
        self
    }

    pub fn spessore_muro(mut self, spessore_muro: Option<u8>) -> Self {
        if spessore_muro.is_none() {
            self.spessore_muro = None;
        } else {
            self.spessore_muro = Some(spessore_muro.unwrap());
        }
        self
    }

    pub fn riscaldamento(mut self, riscaldamento: Option<String>) -> Self {
        if riscaldamento.is_none() {
            self.riscaldamento = None;
        } else {
            self.riscaldamento = Some(riscaldamento.unwrap());
        }
        self
    }

    pub fn raffrescamento(mut self, raffrescamento: Option<String>) -> Self {
        if raffrescamento.is_none() {
            self.raffrescamento = None;
        } else {
            self.raffrescamento = Some(raffrescamento.unwrap());
        }
        self
    }

    pub fn illuminazione(mut self, illuminazione: Option<String>) -> Self {
        if illuminazione.is_none() {
            self.illuminazione = None;
        } else {
            self.illuminazione = Some(illuminazione.unwrap());
        }
        self
    }

    pub fn build(self) -> Result<Stanza, String> {
        Ok(Stanza {
            id: self.id.ok_or("id è obbligatorio")?,
            chiave: self.chiave.ok_or("Il chiave è obbligatorio")?,
            piano: self.piano.ok_or("Il piano è obbligatorio")?,
            id_spazio: self.id_spazio.ok_or("id_spazio è obbligatorio")?,
            stanza: self.stanza.ok_or("stanza è obbligatorio")?,
            destinazione_uso: self
                .destinazione_uso
                .ok_or("destinazione_uso è obbligatorio")?,
            altezza: self.altezza,
            spessore_muro: self.spessore_muro,
            riscaldamento: self.riscaldamento,
            raffrescamento: self.raffrescamento,
            illuminazione: self.illuminazione,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StanzaConInfisso {
    pub id_stanza: u32,
    pub ids_infissi: Vec<String>,
}

#[allow(dead_code)]
impl StanzaConInfisso {
    pub fn new(id_stanza: u32, ids_infissi: Vec<String>) -> Self {
        Self {
            id_stanza,
            ids_infissi,
        }
    }
}
