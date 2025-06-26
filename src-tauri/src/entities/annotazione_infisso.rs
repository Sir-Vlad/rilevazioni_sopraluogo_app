use crate::app_traits::{EntityTrait, FromDto, FromRow, SqlParams, ToInsert, ToRetrieveAll};
use crate::dto::AnnotazioneInfissoDTO;
use crate::entities::utils::convert_timestamp_to_local;
use rusqlite::{Error, Row};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct AnnotazioneInfisso {
    pub(crate) id: u64,
    pub(crate) id_infisso: String,
    pub(crate) edificio: String,
    pub(crate) content: String,
    pub(crate) _data: Option<String>,
}

impl FromDto for AnnotazioneInfisso {
    type Dto = AnnotazioneInfissoDTO;

    fn from_dto(dto: Self::Dto) -> Self {
        Self {
            id: dto.id,
            id_infisso: dto.id_infisso,
            edificio: dto.edificio,
            content: dto.content,
            _data: None,
        }
    }
}

impl FromRow for AnnotazioneInfisso {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let timestamp = row.get("DATA")?;
        let utc = convert_timestamp_to_local(timestamp);

        Ok(Self {
            id: row.get("ID")?,
            id_infisso: row.get("ID_INFISSO")?,
            edificio: row.get("EDIFICIO")?,
            content: row.get("CONTENT")?,
            _data: utc,
        })
    }
}

impl EntityTrait for AnnotazioneInfisso {
    type PrimaryKey = u64;

    #[inline]
    fn table_name() -> String {
        "ANNOTAZIONE_INFISSO".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID         INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_INFISSO TEXT NOT NULL,
                    EDIFICIO   TEXT NOT NULL,
                    CONTENT    TEXT NOT NULL CHECK ( length(CONTENT) > 0 ),
                    DATA       TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (ID_INFISSO, EDIFICIO) REFERENCES INFISSO (ID, EDIFICIO)
                ) STRICT;",
            Self::table_name()
        )
    }
}

impl ToRetrieveAll for AnnotazioneInfisso {}

impl ToInsert for AnnotazioneInfisso {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (ID_INFISSO, EDIFICIO, CONTENT) VALUES (?, ?, ?) RETURNING *;",
            Self::table_name(),
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.id_infisso, &self.edificio, &self.content]
    }
}
