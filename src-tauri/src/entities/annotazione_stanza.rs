use crate::app_traits::{EntityTrait, FromDto, FromRow, SqlParams, ToInsert, ToRetrieveAll};
use crate::dto::AnnotazioneStanzaDTO;
use crate::entities::utils::convert_timestamp_to_local;
use rusqlite::{Error, Row};

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct AnnotazioneStanza {
    pub(crate) id: u64,
    pub(crate) id_stanza: u64,
    pub(crate) content: String,
    pub(crate) _data: Option<String>,
}

impl FromDto for AnnotazioneStanza {
    type Dto = AnnotazioneStanzaDTO;

    fn from_dto(dto: Self::Dto) -> Self {
        Self {
            id: dto.id,
            id_stanza: dto.id_stanza,
            content: dto.content,
            _data: None,
        }
    }
}

impl FromRow for AnnotazioneStanza {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let timestamp = row.get("DATA")?;
        let utc = convert_timestamp_to_local(timestamp);

        Ok(Self {
            id: row.get("ID")?,
            id_stanza: row.get("ID_STANZA")?,
            content: row.get("CONTENT")?,
            _data: if utc.is_none() {
                None
            } else {
                Some(utc.unwrap())
            },
        })
    }
}

impl EntityTrait for AnnotazioneStanza {
    type PrimaryKey = u64;

    #[inline]
    fn table_name() -> String {
        "ANNOTAZIONE_STANZA".to_string()
    }

    #[inline]
    fn sql_create_table() -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {}
                (
                    ID        INTEGER PRIMARY KEY AUTOINCREMENT,
                    ID_STANZA INTEGER  NOT NULL REFERENCES STANZA (ID),
                    CONTENT   TEXT     NOT NULL CHECK ( LENGTH(CONTENT) > 0 ),
                    DATA      TEXT     NOT NULL DEFAULT CURRENT_TIMESTAMP
                ) STRICT;",
            Self::table_name()
        )
    }
}

impl ToRetrieveAll for AnnotazioneStanza {}

impl ToInsert for AnnotazioneStanza {
    #[inline]
    fn to_insert() -> String {
        format!(
            "INSERT INTO {} (ID_STANZA, CONTENT) VALUES (?, ?) RETURNING *;",
            Self::table_name()
        )
    }

    fn to_insert_params(&self) -> Vec<&dyn SqlParams> {
        vec![&self.id_stanza, &self.content]
    }
}
