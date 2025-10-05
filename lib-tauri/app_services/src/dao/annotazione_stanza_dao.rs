use app_models::{
    models::{AnnotazioneStanza, NewAnnotazioneStanza},
    schema::annotazione_stanza,
};
use app_utils::{
    app_error::DomainError,
    app_interface::{
        dao_interface::{
            DAO,
            crud_operations::{GetAll, Insert},
        },
        database_interface::PostgresPooled,
    },
};
use diesel::RunQueryDsl;

use crate::dao::utils::map_error_annotazione;

pub struct AnnotazioneStanzaDAO;

impl DAO for AnnotazioneStanzaDAO {}

impl GetAll<AnnotazioneStanza> for AnnotazioneStanzaDAO {
    type Output = AnnotazioneStanza;

    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        annotazione_stanza::table
            .load(conn)
            .map_err(DomainError::from)
    }
}

impl Insert<NewAnnotazioneStanza> for AnnotazioneStanzaDAO {
    type Output = AnnotazioneStanza;

    fn insert(
        conn: &mut PostgresPooled,
        item: NewAnnotazioneStanza,
    ) -> Result<Self::Output, DomainError> {
        diesel::insert_into(annotazione_stanza::table)
            .values(&item)
            .get_result(conn)
            .map_err(map_error_annotazione)
    }
}

#[cfg(test)]
mod test {
    use app_models::models::NewAnnotazioneStanza;
    use app_utils::{
        app_error::{DomainError, ErrorKind::EmptyField},
        app_interface::dao_interface::crud_operations::Insert,
        test::create_postgres_pool,
    };

    use crate::dao::AnnotazioneStanzaDAO;

    #[tokio::test]
    async fn test_insert() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();
        let item = NewAnnotazioneStanza {
            stanza_id: 1,
            content: "test".to_string(),
        };
        match AnnotazioneStanzaDAO::insert(&mut conn, item.clone()) {
            Ok(res) => assert_eq!(res.stanza_id, item.stanza_id),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_insert_content_empty() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();
        let item = NewAnnotazioneStanza {
            stanza_id: 1,
            content: "".to_string(),
        };
        match AnnotazioneStanzaDAO::insert(&mut conn, item.clone()) {
            Ok(_) => panic!("Should not be able to insert"),
            Err(e) => {
                println!("{e:?}");
                assert_eq!(
                    e,
                    DomainError::InvalidInput(
                        EmptyField,
                        "Field content cannot be empty or contain only whitespace".to_string()
                    )
                )
            }
        }
    }
}
