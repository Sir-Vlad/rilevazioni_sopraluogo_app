use crate::app_traits::{CreateTable, DaoTrait, GetAll, Insert, Update};
use crate::entities::Utenza;
use crate::utils::AppError;

pub struct UtenzeDAO;

impl DaoTrait for UtenzeDAO {
    type Entity = Utenza;
    type Error = AppError;
}
impl CreateTable for UtenzeDAO {}
impl GetAll for UtenzeDAO {}
impl Insert for UtenzeDAO {}
impl Update for UtenzeDAO {}

/*
impl DAO for UtenzeDAO {
    fn table_name() -> &'static str {
        "UTENZE"
    }
}

impl CreateTable for UtenzeDAO {
    fn create_table<C: DatabaseConnection>(conn: &C) -> Result<(), AppError> {
        conn.execute(
            format!("CREATE TABLE IF NOT EXISTS {}
            (
                ID                  INTEGER PRIMARY KEY AUTOINCREMENT,
                ID_EDIFICIO         TEXT NOT NULL REFERENCES EDIFICIO (CHIAVE),
                TIPO                TEXT NOT NULL CHECK ( TIPO IN ('idrica', 'termica', 'elettrica') ),
                COD_CONTATORE       TEXT NOT NULL,
                INDIRIZZO_CONTATORE TEXT
            ) STRICT;", Self::table_name()).as_str(),
            (),
        )?;
        info!("Table utenze creata");
        Ok(())
    }
}

impl GetAll<Utenza> for UtenzeDAO {
    fn get_all<C: DatabaseConnection>(conn: &C) -> Result<Vec<Utenza>, AppError> {
        let (query, _) = QueryBuilder::select().table("UTENZE").build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let result: Result<Vec<Utenza>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Utenza {
                    id: row.get("ID")?,
                    id_edificio: row.get("ID_EDIFICIO")?,
                    tipo: row.get::<_, String>("TIPO")?.into(),
                    cod_contatore: row.get("COD_CONTATORE")?,
                    indirizzo_contatore: row.get("INDIRIZZO_CONTATORE")?,
                })
            })?
            .collect();
        result.map_err(AppError::from)
    }
}

impl Insert<Utenza> for UtenzeDAO {
    fn insert<C: DatabaseConnection>(conn: &C, item: Utenza) -> Result<Utenza, AppError> {
        let builder = QueryBuilder::insert()
            .table("UTENZE")
            .columns(vec![
                "ID_EDIFICIO",
                "TIPO",
                "COD_CONTATORE",
                "INDIRIZZO_CONTATORE",
            ])
            .values(vec![
                item.id_edificio.clone().into(),
                item.tipo.to_string().into(),
                item.cod_contatore.clone().into(),
                item.indirizzo_contatore.clone().into(),
            ])
            .returning("ID");
        let (query, param) = builder.build()?;
        let mut stmt = conn.prepare(query.as_str())?;
        let mut result = stmt
            .query_map(rusqlite::params_from_iter(convert_param(param)), |row| {
                row.get::<_, u64>(0)
            })?;
        let id = result.next().unwrap()?;
        info!("Utenza inserita con id {}", id);
        Ok(Utenza { id, ..item })
    }
}

impl Update<Utenza> for UtenzeDAO {
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
}
*/

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::app_traits::{CreateTable, Insert, Update};
    use crate::entities::{Edificio, Utenza};
    use rusqlite::Connection;

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
    fn test_insert() {
        let conn = setup();
        let utenza = Utenza::new("PR01-25", "idrica", "00008545", "Via Pallone");

        match UtenzeDAO::insert(&conn, utenza) {
            Ok(res) => {
                assert_eq!(1, res.id);
            }
            Err(e) => panic!("{}", e),
        }

        pretty_sqlite::print_table(&conn, "UTENZE").unwrap();
    }

    #[test]
    fn test_update() {
        let conn = setup();
        let mut utenza = Utenza::new("PR01-25", "idrica", "00008545", "Via Pallone");
        UtenzeDAO::insert(&conn, utenza.clone()).unwrap();

        pretty_sqlite::print_table(&conn, "UTENZE").unwrap();

        utenza.id = 1;
        utenza.cod_contatore = "00008545".to_string();
        utenza.indirizzo_contatore = Option::from("Via Roma".to_string());

        let res = UtenzeDAO::update(&conn, utenza.clone());
        match res {
            Ok(res) => {
                pretty_sqlite::print_table(&conn, "UTENZE").unwrap();
                assert_eq!(res, utenza);
            }
            Err(e) => panic!("{}", e),
        }
    }
}
