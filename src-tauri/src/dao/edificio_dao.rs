use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert, Update};
use crate::dao::entity::Edificio;
use crate::utils::AppError;

pub struct EdificioDAO;

impl DaoTrait for EdificioDAO {
    type Entity = Edificio;
    type Error = AppError;
}

impl CreateTable for EdificioDAO {}

impl GetAll for EdificioDAO {}

impl Insert for EdificioDAO {}

impl Update for EdificioDAO {}

/*
impl Insert<Edificio> for EdificioDAO {
    fn insert<C: DatabaseConnection>(conn: &C, item: Edificio) -> Result<Edificio, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec!["CHIAVE", "FASCICOLO", "INDIRIZZO"])
            .values(vec![
                item.chiave.clone().into(),
                item.fascicolo.clone().into(),
                item.indirizzo.clone().into(),
            ]);
        let (query, params) = builder.build()?;

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(params)),
        ) {
            Ok(_) => {
                info!("Edificio {} inserito con successo", item.chiave);
                Ok(item)
            }
            Err(e) => {
                error!("Errore durante l'inserimento {{ edificio }}: {}", e);
                Err(e)
            }
        }
    }
}

impl Update<Edificio> for EdificioDAO {
    fn update<C: DatabaseConnection>(conn: &C, item: Edificio) -> Result<Edificio, AppError> {
        let builder = QueryBuilder::update()
            .table(Self::table_name())
            .set_if("ANNO_COSTRUZIONE", item.anno_costruzione.clone())
            .set_if("ANNO_RIQUALIFICAZIONE", item.anno_riqualificazione.clone())
            .set_if("NOTE_RIQUALIFICAZIONE", item.note_riqualificazione.clone())
            .set_if("ISOLAMENTO_TETTO", item.isolamento_tetto)
            .set_if("CAPPOTTO", item.cappotto)
            .where_eq("CHIAVE", item.chiave.clone());

        let (query, param) = builder.build()?;

        match conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(param)),
        ) {
            Ok(_) => {
                info!("Edificio aggiornato con successo");
                Ok(item)
            }
            Err(e) => {
                error!("Errore durante l'aggiornamento {{ edificio }}: {}", e);
                Err(e)
            }
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_traits::EntityTrait;
    use rusqlite::Connection;

    #[test]
    fn test_insert() {
        let conn = Connection::open_in_memory().unwrap();
        EdificioDAO::create_table(&conn).unwrap();

        let entity = Edificio {
            chiave: "TEST01".to_string(),
            fascicolo: "FD456".to_string(),
            indirizzo: "Via Test, 45".to_string(),
            anno_costruzione: None,
            anno_riqualificazione: None,
            note_riqualificazione: None,
            isolamento_tetto: None,
            cappotto: None,
        };

        match EdificioDAO::insert(&conn, entity.clone()) {
            Ok(res) => {
                assert_eq!(res.chiave, entity.chiave);
            }
            Err(err) => panic!("Errore durante l'inserimento: {}", err),
        }

        pretty_sqlite::print_table(&conn, &Edificio::table_name()).unwrap()
    }

    #[test]
    fn test_update() {
        let conn = Connection::open_in_memory().unwrap();
        EdificioDAO::create_table(&conn).unwrap();

        let mut entity = Edificio {
            chiave: "TEST01".to_string(),
            fascicolo: "FD456".to_string(),
            indirizzo: "Via Test, 45".to_string(),
            anno_costruzione: None,
            anno_riqualificazione: None,
            note_riqualificazione: None,
            isolamento_tetto: None,
            cappotto: None,
        };

        EdificioDAO::insert(&conn, entity.clone()).unwrap();

        pretty_sqlite::print_table(&conn, &Edificio::table_name()).unwrap();

        entity.anno_costruzione = Some("1955".to_string());
        entity.isolamento_tetto = Some(true);

        match EdificioDAO::update(&conn, entity.clone()) {
            Ok(res) => {
                assert_eq!(res.chiave, entity.chiave);
            }
            Err(err) => panic!("Errore durante l'aggiornamento: {}", err),
        }

        pretty_sqlite::print_table(&conn, &Edificio::table_name()).unwrap()
    }
}
