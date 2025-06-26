use crate::app_traits::{EntityTrait, FromDto, FromRow, SqlParams, ToInsert, ToRetrieveAll};
use crate::dto::AnnotazioneEdificioDTO;
use crate::entities::utils::convert_timestamp_to_local;
use rusqlite::Row;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct AnnotazioneEdificio {
    pub(crate) id: u64,
    pub(crate) id_edificio: String,
    pub(crate) content: String,
    pub(crate) _data: Option<String>,
}

impl FromDto for AnnotazioneEdificio {
    type Dto = AnnotazioneEdificioDTO;

    fn from_dto(dto: Self::Dto) -> Self {
        Self {
            id: dto.id,
            id_edificio: dto.id_edificio,
            content: dto.content,
            _data: None,
        }
    }
}

impl FromRow for AnnotazioneEdificio {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        let timestamp = row.get("DATA")?;
        let utc = convert_timestamp_to_local(timestamp);

        Ok(Self {
            id: row.get("ID")?,
            id_edificio: row.get("ID_EDIFICIO")?,
            content: row.get("CONTENT")?,
            _data: utc,
        })
    }
}

impl EntityTrait for AnnotazioneEdificio {
    type PrimaryKey = u64;

    #[inline]
    fn table_name() -> String {
        "ANNOTAZIONI_EDIFICIO".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID          INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_EDIFICIO TEXT REFERENCES EDIFICIO (CHIAVE),
                    CONTENT     TEXT NOT NULL CHECK ( length(CONTENT) > 0 ),
                    DATA        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                ) STRICT;",
            Self::table_name()
        )
    }
}

impl ToRetrieveAll for AnnotazioneEdificio {}

impl ToInsert for AnnotazioneEdificio {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (ID_EDIFICIO, CONTENT) VALUES (?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.id_edificio, &self.content]
    }
}
