use app_error::DomainError;
use app_interface::dao_interface::crud_operations::{Get, GetAll, Insert, Update};
use app_interface::dao_interface::DAO;
use app_interface::database_interface::PostgresPooled;
use app_models::models::StanzaConInfissi;
use app_models::schema::stanza_con_infissi;
use diesel::associations::HasTable;
use diesel::result::Error;
use diesel::{EqAll, ExpressionMethods, QueryDsl, RunQueryDsl};
use std::collections::HashSet;

pub struct StanzaConInfissiDao;

impl DAO for StanzaConInfissiDao {
    
}

impl Get<StanzaConInfissi, (String, i32)> for StanzaConInfissiDao {
    type Output = Vec<StanzaConInfissi>;
    /// L'id è una tuple di id che corrispondono -> (edificio, stanza)
    fn get(conn: &mut PostgresPooled, id: (String, i32)) -> Result<Self::Output, DomainError> {
        stanza_con_infissi::table
            .filter(stanza_con_infissi::edificio_id.eq(id.0))
            .filter(stanza_con_infissi::stanza_id.eq(id.1))
            .get_results(conn)
            .map_err(|e| match e {
                Error::NotFound => DomainError::StanzaConInfissiNotFound,
                _ => DomainError::Unexpected(e),
            })
    }

    /*
    fn get<C: DatabaseConnection>(
        conn: &C,
        id: (u64, String),
    ) -> Result<StanzaConInfissi, AppError> {
        let (id, edificio) = id;
        let builder = QueryBuilder::select()
            .table(Self::table_name())
            .where_eq("ID_STANZA", id)
            .where_eq("ID_EDIFICIO", edificio.clone());
        let (query, _) = builder.build()?;

        let mut stmt = conn.prepare(query.as_str())?;
        let result: Result<Vec<(String, u64)>, Error> = stmt
            .query_map(params![id, edificio], |row| {
                let id_infisso = row.get("ID_INFISSO")?;
                let ripetizioni = row.get("NUM_INFISSI")?;
                Ok((id_infisso, ripetizioni))
            })?
            .collect();

        match result {
            Ok(infissi) => {
                if infissi.is_empty() {
                    Err(AppError::NotFound(format!(
                        "Stanza con ID {} non trovata",
                        id
                    )))
                } else {
                    Ok(StanzaConInfissi::new(id, infissi, edificio))
                }
            }
            Err(e) => Err(AppError::from(e)),
        }
    }

     */
}

impl Insert<StanzaConInfissi> for StanzaConInfissiDao {
    type Output = StanzaConInfissi;
    fn insert(
        conn: &mut PostgresPooled,
        item: StanzaConInfissi,
    ) -> Result<Self::Output, DomainError> {
        diesel::insert_into(stanza_con_infissi::table)
            .values(&item)
            .get_result(conn)
            .map_err(DomainError::from)
    }

    /*
    fn insert<C: DatabaseConnection>(
        conn: &C,
        item: StanzaConInfissi,
    ) -> Result<StanzaConInfissi, AppError> {
        let builder = QueryBuilder::insert()
            .table(Self::table_name())
            .columns(vec![
                "ID_STANZA",
                "ID_INFISSO",
                "ID_EDIFICIO",
                "NUM_INFISSI",
            ])
            .values(vec![0.into(), "A".into(), "".into(), 0.into()]); // param fake
        let (query, _) = builder.build()?;

        let mut stmt = conn.prepare(query.as_str())?;
        for (id_infisso, num_infisso) in item.id_infissi.clone() {
            stmt.execute(params![
                item.id_stanza,
                id_infisso,
                item.id_edificio,
                num_infisso
            ])?;
        }

        Ok(item)
    }

     */
}

impl Update<StanzaConInfissi, (String, String, i32)> for StanzaConInfissiDao {
    type Output = StanzaConInfissi;
    fn update(
        conn: &mut PostgresPooled,
        id: (String, String, i32),
        item: StanzaConInfissi,
    ) -> Result<Self::Output, DomainError> {
        todo!()
    }
    /*
    fn update<C: DatabaseConnection>(
        conn: &C,
        item: StanzaConInfissi,
    ) -> Result<StanzaConInfissi, AppError> {
        // Recupero l'elemento esistente per confrontarlo con quello nuovo
        let existing = match Self::get(conn, (item.id_stanza, item.id_edificio.clone())) {
            Ok(existing) => existing,
            Err(AppError::NotFound(_)) => {
                // Se non esiste, facciamo direttamente l'insert
                return Self::insert(conn, item);
            }
            Err(e) => return Err(e),
        };

        // Troviamo i dati comuni e quelli unici
        let (common, unique) =
            find_common_and_unique(existing.id_infissi.clone(), item.id_infissi.clone());

        // Per ogni infisso comune, aggiorniamo la quantità
        for id_infisso in common {
            // Troviamo la nuova quantità di infissi
            let new_quantity = item
                .id_infissi
                .iter()
                .find(|(id, _)| id == &id_infisso)
                .map(|(_, qty)| *qty)
                .unwrap_or(0);
            // Troviamo la vecchia quantità di infissi
            let old_quantity = existing
                .id_infissi
                .iter()
                .find(|(id, _)| id == &id_infisso)
                .map(|(_, qty)| *qty)
                .unwrap_or(0);
            // Aggiorniamo il numero di infissi
            let builder = QueryBuilder::update()
                .table(Self::table_name())
                .set("NUM_INFISSI", new_quantity + old_quantity)
                .where_eq("ID_STANZA", item.id_stanza)
                .where_eq("ID_INFISSO", id_infisso);

            let (query, param) = builder.build()?;
            conn.execute(
                query.as_str(),
                rusqlite::params_from_iter(convert_param(param)),
            )?;
        }

        // Per gli infissi unici nella nuova lista che non sono nel DB, li inseriamo
        if !unique.is_empty() {
            let infissi: Vec<(String, u64)> = item
                .id_infissi
                .iter()
                .filter(|x| unique.iter().any(|x1| x1.eq(&x.0)))
                .cloned()
                .collect();

            let infisso = StanzaConInfissi::new(item.id_stanza, infissi, item.id_edificio.clone());

            Self::insert(conn, infisso)?;
        }

        // Restituiamo l'item aggiornato
        Ok(item)
    }

     */
}

fn find_common_and_unique(
    elements: Vec<(String, u64)>,
    items: Vec<(String, u64)>,
) -> (Vec<String>, Vec<String>) {
    let set_elements: HashSet<_> = elements.iter().map(|(x, _)| x).collect();
    let set_items: HashSet<_> = items.iter().map(|(x, _)| x).collect();

    let common = set_elements
        .intersection(&set_items)
        .map(|item| (*item).clone())
        .collect();

    let unique = set_elements
        .symmetric_difference(&set_items)
        .map(|item| (*item).clone())
        .collect();

    (common, unique)
}

#[cfg(test)]
mod test {
    use super::*;
    use app_utils::test::create_postgres_pool;

    static ID_EDIFICIO: &str = "PR01-25";

    #[tokio::test]
    async fn test_get_stanza_con_infissi() {
        let pool = create_postgres_pool().await;
        let mut conn = pool.get().unwrap();

        match StanzaConInfissiDao::get(&mut conn, ("9338-14".to_string(), 30)) {
            Ok(stanza_con_infissi) => {
                assert_eq!(stanza_con_infissi.len(), 3);
                println!("{stanza_con_infissi:#?}")
            }
            Err(e) => {
                panic!("{e:?}")
            }
        }
    }

    /*
    #[test]
    fn create_table() {
        let conn = Connection::open_in_memory().unwrap();
        let res = StanzaConInfissiDao::create_table(&conn);
        assert!(res.is_ok());
    }

    #[test]
    fn insert_value() {
        let conn = DATABASE.lock().unwrap();
        let stanza_con_infissi = StanzaConInfissi {
            id_stanza: 1,
            id_infissi: vec![("A".to_string(), 5), ("B".to_string(), 8)],
            id_edificio: ID_EDIFICIO.to_string(),
        };

        let res = StanzaConInfissiDao::insert(conn.deref(), stanza_con_infissi.clone());
        match res {
            Ok(r) => {
                pretty_sqlite::print_table(&conn, StanzaConInfissiDao::table_name()).unwrap();
                assert_eq!(r, stanza_con_infissi);
            }
            Err(e) => eprintln!("Errore: {}", e),
        }
    }

    #[test]
    #[serial]
    fn get_stanza_con_infissi() {
        let conn = DATABASE.lock().unwrap();

        let res = StanzaConInfissiDao::get(conn.deref(), (1, ID_EDIFICIO.to_string()));
        match res {
            Ok(r) => {
                pretty_sqlite::print_table(&conn, StanzaConInfissiDao::table_name()).unwrap();
                assert_eq!(
                    r,
                    StanzaConInfissi {
                        id_stanza: 1,
                        id_infissi: vec![("A".to_string(), 5), ("B".to_string(), 8)],
                        id_edificio: ID_EDIFICIO.to_string()
                    }
                );
            }
            Err(e) => eprintln!("Errore: {}", e),
        }
    }

    #[test]
    fn get_all_stanza_con_infissi() {
        let conn = setup();

        StanzaConInfissiDao::insert(
            &conn,
            StanzaConInfissi::new(
                1,
                vec![("A".to_string(), 5), ("B".to_string(), 2)],
                ID_EDIFICIO.to_string(),
            ),
        )
        .unwrap();
        StanzaConInfissiDao::insert(
            &conn,
            StanzaConInfissi::new(
                2,
                vec![("A".to_string(), 2), ("B".to_string(), 1)],
                ID_EDIFICIO.to_string(),
            ),
        )
        .unwrap();

        pretty_sqlite::print_table(&conn, StanzaConInfissiDao::table_name()).unwrap();

        let res = StanzaConInfissiDao::get_all(&conn);
        match res {
            Ok(r) => {
                pretty_sqlite::print_table(&conn, StanzaConInfissiDao::table_name()).unwrap();
                assert_eq!(r.len(), 2);
                println!("{:?}", r);
            }
            Err(e) => eprintln!("Errore: {}", e),
        }
    }

    #[test]
    fn update_stanza_con_infissi() {
        let conn = setup();

        StanzaConInfissiDao::insert(
            &conn,
            StanzaConInfissi::new(
                1,
                vec![("A".to_string(), 5), ("B".to_string(), 8)],
                ID_EDIFICIO.to_string(),
            ),
        )
        .unwrap();
        StanzaConInfissiDao::insert(
            &conn,
            StanzaConInfissi::new(2, vec![("A".to_string(), 2)], ID_EDIFICIO.to_string()),
        )
        .unwrap();

        pretty_sqlite::print_table(&conn, StanzaConInfissiDao::table_name()).unwrap();

        let res = StanzaConInfissiDao::update(
            &conn,
            StanzaConInfissi::new(
                2,
                vec![("A".to_string(), 1), ("B".to_string(), 2)],
                ID_EDIFICIO.to_string(),
            ),
        );
        match res {
            Ok(r) => {
                pretty_sqlite::print_table(&conn, StanzaConInfissiDao::table_name()).unwrap();
                println!("{:?}", r);
            }
            Err(e) => {
                panic!("Errore: {}", e);
            }
        }
    }

     */
}
