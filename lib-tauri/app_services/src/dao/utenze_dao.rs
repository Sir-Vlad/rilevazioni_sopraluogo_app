use crate::service::Get;
use app_models::models::{NewUtenza, UpdateUtenza, Utenza};
use app_models::schema::utenze;
use app_utils::app_error::DomainError;
use app_utils::app_interface::dao_interface::crud_operations::{GetAll, Insert, Update};
use app_utils::app_interface::dao_interface::DAO;
use app_utils::app_interface::database_interface::PostgresPooled;
use diesel::result::Error;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};

pub struct UtenzeDAO;

impl DAO for UtenzeDAO {}

impl GetAll<Utenza> for UtenzeDAO {
    type Output = Utenza;
    fn get_all(conn: &mut PostgresPooled) -> Result<Vec<Self::Output>, DomainError> {
        utenze::table.load(conn).map_err(DomainError::from)
    }
}

/// Retrieve per edificio
impl Get<Utenza, String> for UtenzeDAO {
    type Output = Vec<Utenza>;

    fn get(conn: &mut PostgresPooled, id: String) -> Result<Self::Output, DomainError> {
        utenze::table
            .filter(utenze::edificio_id.eq(id))
            .get_results(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::UtenzaNotFound,
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Insert<NewUtenza> for UtenzeDAO {
    type Output = Utenza;
    fn insert(conn: &mut PostgresPooled, item: NewUtenza) -> Result<Self::Output, DomainError> {
        diesel::insert_into(utenze::table)
            .values(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::UtenzaNotFound,
                Error::DatabaseError(kind, ..) => {
                    if matches!(kind, diesel::result::DatabaseErrorKind::UniqueViolation) {
                        DomainError::UtenzaAlreadyExists
                    } else {
                        DomainError::from(e)
                    }
                }
                _ => DomainError::Unexpected(e),
            })
    }
}

impl Update<UpdateUtenza, i32> for UtenzeDAO {
    type Output = Utenza;

    fn update(
        conn: &mut PostgresPooled,
        id: i32,
        item: UpdateUtenza,
    ) -> Result<Self::Output, DomainError> {
        diesel::update(utenze::table)
            .set(&item)
            .get_result(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::StanzaNotFound,
                _ => DomainError::Unexpected(e),
            })
    }

    /*
    fn update<C: DatabaseConnection>(conn: &C, item: Utenza) -> Result<Utenza, AppError> {
        let builder = QueryBuilder::update()
            .table("UTENZE")
            .set("COD_CONTATORE", item.cod_contatore.clone())
            .set_if("INDIRIZZO_CONTATORE", item.indirizzo_contatore.clone())
            .where_eq("ID", item.id);
        let (query, params) = builder.build()?;
        conn.execute(
            query.as_str(),
            rusqlite::params_from_iter(convert_param(params)),
        )?;
        Ok(item)
    }

     */
}

#[cfg(test)]
mod tests {
    /*
    use crate::dao::crud_operations::{Insert, Update};
    use crate::dao::entity::{Edificio, TipoUtenza, Utenza};
    use crate::dao::utils::schema_operations::CreateTable;
    use app_utils::app_interface::dao_interface::crud_operations::Insert;
    use app_models::models::{Edificio, Utenza};
    use diesel::Connection;
    use once_cell::sync::Lazy;
    use rusqlite::Connection;
    use serial_test::serial;
    use std::ops::Deref;
    use std::sync::Mutex;

    static DATABASE: Lazy<Mutex<Connection>> = Lazy::new(|| Mutex::new(setup()));

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        EdificioDAO::create_table(&conn).expect("Errore nella creazione della tabella Edificio");
        UtenzeDAO::create_table(&conn).expect("Errore nella creazione della tabella Utenze");

        let edificio = Edificio::new("PR01-25", "00008545", "Via Pallone");
        EdificioDAO::insert(&conn, edificio).expect("Errore nella creazione dell'edificio");
        conn
    }

    #[test]
    fn test_create_table() {
        let conn = Connection::open_in_memory().unwrap();

        let res = UtenzeDAO::create_table(&conn);
        assert!(res.is_ok());
    }

    #[test]
    #[serial]
    fn test_insert() {
        let conn = DATABASE.lock().unwrap();
        let utenza = Utenza::new("PR01-25", "acqua", "00008545", "Via Pallone");
        let result = UtenzeDAO::insert(conn.deref(), utenza);
        match result {
            Ok(res) => {
                pretty_sqlite::print_table(&conn, "UTENZE").unwrap();
                assert_eq!(
                    res,
                    Utenza {
                        id: 1,
                        id_edificio: "PR01-25".to_string(),
                        tipo: TipoUtenza::Idrica,
                        cod_contatore: "00008545".to_string(),
                        indirizzo_contatore: Option::from("Via Pallone".to_string()),
                    }
                )
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[serial]
    fn test_update() {
        let conn = DATABASE.lock().unwrap();
        let update_utenza = Utenza {
            id: 1,
            id_edificio: "PR01-25".to_string(),
            tipo: TipoUtenza::Idrica,
            cod_contatore: "00008545".to_string(),
            indirizzo_contatore: Option::from("Via Roma".to_string()),
        };
        pretty_sqlite::print_table(&conn, "UTENZE").unwrap();
        let res = UtenzeDAO::update(conn.deref(), update_utenza.clone());
        match res {
            Ok(res) => {
                pretty_sqlite::print_table(&conn, "UTENZE").unwrap();
                assert_eq!(res, update_utenza);
            }
            Err(e) => panic!("{}", e),
        }
    }

     */
}
