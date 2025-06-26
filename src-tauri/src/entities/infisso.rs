use crate::app_traits::{
    EntityTrait, FromDto, FromRow, SqlParams, ToInsert, ToRetrieveAll, ToUpdate,
};
use crate::dto::InfissoDTO;
use rusqlite::{Error, Row};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone)]
pub struct Infisso {
    pub(crate) id: String,
    pub(crate) edificio_id: String,
    pub(crate) tipo: String,
    pub(crate) altezza: u16,
    pub(crate) larghezza: u16,
    pub(crate) materiale: String,
    pub(crate) vetro: String,
}

impl Infisso {
    #[cfg(test)]
    pub fn new(
        id: &str,
        edificio_id: &str,
        tipo: &str,
        altezza: u16,
        larghezza: u16,
        materiale: &str,
        vetro: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            edificio_id: edificio_id.to_string(),
            tipo: tipo.to_string(),
            altezza,
            larghezza,
            materiale: materiale.to_string(),
            vetro: vetro.to_string(),
        }
    }
}

impl FromDto for Infisso {
    type Dto = InfissoDTO;

    fn from_dto(dto: Self::Dto) -> Self {
        Self {
            id: dto.id,
            edificio_id: dto.id_edificio,
            tipo: dto.tipo,
            altezza: dto.altezza,
            larghezza: dto.larghezza,
            materiale: dto.materiale,
            vetro: dto.vetro,
        }
    }
}

impl FromRow for Infisso {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row.get("ID")?,
            edificio_id: row.get("EDIFICIO")?,
            tipo: row.get("TIPO")?,
            altezza: row.get("ALTEZZA")?,
            larghezza: row.get("LARGHEZZA")?,
            materiale: row.get("MATERIALE")?,
            vetro: row.get("VETRO")?,
        })
    }
}

impl EntityTrait for Infisso {
    type PrimaryKey = (String, String);

    #[inline]
    fn table_name() -> String {
        "INFISSO".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID        TEXT,
                    EDIFICIO  TEXT    NOT NULL REFERENCES EDIFICIO (CHIAVE),
                    TIPO      TEXT    NOT NULL REFERENCES TIPO_INFISSO (NOME),
                    ALTEZZA   INTEGER NOT NULL CHECK ( ALTEZZA >= 0 ),
                    LARGHEZZA INTEGER NOT NULL CHECK ( LARGHEZZA >= 0 ),
                    MATERIALE TEXT    NOT NULL REFERENCES MATERIALE_INFISSO (MATERIALE),
                    VETRO     TEXT    NOT NULL REFERENCES VETRO_INFISSO (VETRO),
                    MQ        REAL GENERATED ALWAYS AS ((ALTEZZA * LARGHEZZA) / 10000.0) VIRTUAL,
                    PRIMARY KEY (ID, EDIFICIO),
                    UNIQUE (ID, EDIFICIO, TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO)
                ) STRICT;",
            Self::table_name()
        )
    }
}

impl ToRetrieveAll for Infisso {}

impl ToInsert for Infisso {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {}(ID, EDIFICIO, TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![
            &self.id,
            &self.edificio_id,
            &self.tipo,
            &self.altezza,
            &self.larghezza,
            &self.materiale,
            &self.vetro,
        ]
    }
}

impl ToUpdate for Infisso {
    #[inline]
    fn to_update() -> String {
        format!(
            "UPDATE {} SET TIPO = ?, ALTEZZA = ?, LARGHEZZA = ?, MATERIALE = ?, VETRO = ? WHERE ID = ? AND EDIFICIO = ? RETURNING *;",
            Self::table_name()
        )
    }

    fn to_update_params(&self) -> Vec<Box<&dyn SqlParams>> {
        vec![
            Box::new(&self.tipo),
            Box::new(&self.altezza),
            Box::new(&self.larghezza),
            Box::new(&self.materiale),
            Box::new(&self.vetro),
            Box::new(&self.id),
            Box::new(&self.edificio_id),
        ]
    }
}
