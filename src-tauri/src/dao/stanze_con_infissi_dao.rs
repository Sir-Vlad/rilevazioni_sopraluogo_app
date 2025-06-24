use crate::app_traits::{
    CreateTable, DaoTrait, EntityTrait, GetAll, GetById, Insert, SqlExecutor, SqlParams, ToInsert,
    ToRetrieve, ToRetrieveAll, ToUpdate, Update,
};
use crate::dao::crud_operations::{
    Get as oldG, Get, GetAll as oldGA, Insert as oldI, Update as oldU,
};
use crate::dao::entity::StanzaConInfissi;
use crate::dao::schema_operations::CreateTable as OldC;
use crate::dao::utils::DAO;
use crate::database::{DatabaseConnection, QueryBuilderError, SqlQueryBuilder, WhereBuilder};
use crate::utils::AppError;
use rusqlite::{params, Error};
use std::collections::{HashMap, HashSet};

pub struct StanzaConInfissiDao;

impl DaoTrait for StanzaConInfissiDao {
    type Entity = StanzaConInfissi;
    type Error = AppError;
}
impl CreateTable for StanzaConInfissiDao {}
impl GetById for StanzaConInfissiDao {
    fn get_by_id<Connection: SqlExecutor>(
        conn: &Connection,
        id: <Self::Entity as EntityTrait>::PrimaryKey,
    ) -> Result<Self::Entity, Self::Error> {
        let query = Self::Entity::to_retrieve();
        let params = id.to_sql_params();

        let mut stmt = conn.prepare(query.as_str())?;
        let result: Result<Vec<(String, u64)>, Error> = stmt
            .query_map(rusqlite::params_from_iter(params), |row| {
                let id_infisso = row.get("ID_INFISSO")?;
                let ripetizioni = row.get("NUM_INFISSI")?;
                Ok((id_infisso, ripetizioni))
            })?
            .collect();

        match result {
            Ok(infissi) => {
                let (id_stanza, id_edificio) = id;
                if infissi.is_empty() {
                    Err(AppError::NotFound(format!(
                        "Stanza con ID ( id_stanza: {}, id_edificio: {} ) non trovata",
                        id_stanza, id_edificio
                    )))
                } else {
                    Ok(StanzaConInfissi::new(id_stanza, infissi, id_edificio))
                }
            }
            Err(e) => Err(AppError::from(e)),
        }
    }
}
impl GetAll for StanzaConInfissiDao {
    fn get_all<Connection: SqlExecutor>(
        conn: &Connection,
    ) -> Result<Vec<Self::Entity>, Self::Error> {
        let query = Self::Entity::to_retrieve_all();
        let mut stmt = conn.prepare(query.as_str())?;

        let mut infissi: HashMap<(String, String), Vec<(String, u64)>> = HashMap::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let id_stanza: u64 = row.get("ID_STANZA")?;
            let id_infisso: String = row.get("ID_INFISSO")?;
            let id_edificio: String = row.get("ID_EDIFICIO")?;
            let num_infissi: u64 = row.get("NUM_INFISSI")?;

            let stanza_infissi = infissi
                .entry((id_stanza.to_string(), id_edificio))
                .or_default();

            stanza_infissi.push((id_infisso, num_infissi));
        }

        let mut result = Vec::new();
        for entry in infissi.into_iter() {
            result.push(StanzaConInfissi::new(
                entry.0.0.parse::<u64>().unwrap(),
                entry.1,
                entry.0.1,
            ));
        }
        result.reverse();
        Ok(result)
    }
}
impl Insert for StanzaConInfissiDao {
    fn insert<Connection: SqlExecutor>(
        conn: &Connection,
        entity: Self::Entity,
    ) -> Result<Self::Entity, Self::Error> {
        let query = Self::Entity::to_insert();

        let mut stmt = conn.prepare(query.as_str())?;
        for (id_infisso, num_infisso) in entity.id_infissi.clone() {
            stmt.execute(params![
                entity.id_stanza,
                id_infisso,
                entity.id_edificio,
                num_infisso
            ])?;
        }

        Ok(entity)
    }
}
impl Update for StanzaConInfissiDao
where
    Self: GetById,
{
    fn update<Connection: SqlExecutor>(
        conn: &Connection,
        entity: Self::Entity,
    ) -> Result<Self::Entity, Self::Error>
    where
        Self::Error: From<QueryBuilderError>,
    {
        // Recupero l'elemento esistente per confrontarlo con quello nuovo
        let existing = match Self::get_by_id(conn, (entity.id_stanza, entity.id_edificio.clone())) {
            Ok(existing) => existing,
            Err(AppError::NotFound(_)) => {
                // Se non esiste, facciamo direttamente l'insert
                return Self::insert(conn, entity);
            }
            Err(e) => return Err(e),
        };

        // Troviamo i dati comuni e quelli unici
        let (common, unique) =
            find_common_and_unique(existing.id_infissi.clone(), entity.id_infissi.clone());
        let query = Self::Entity::to_update();
        let mut stmt = conn.prepare(query.as_str())?;

        // Per ogni infisso comune, aggiorniamo la quantità
        for id_infisso in common {
            // Troviamo la nuova quantità di infissi
            let new_quantity = entity
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
            let new_quantity = new_quantity + old_quantity;

            let params: Vec<Box<&dyn SqlParams>> = vec![
                Box::new(&new_quantity),
                Box::new(&id_infisso),
                Box::new(&entity.id_stanza),
                Box::new(&entity.id_edificio),
            ];
            let sql_params = params
                .into_iter()
                .flat_map(|p| p.to_sql_params())
                .collect::<Vec<_>>();

            stmt.execute(rusqlite::params_from_iter(sql_params))?;
        }

        // Per gli infissi unici nella nuova lista che non sono nel DB, li inseriamo
        if !unique.is_empty() {
            let infissi: Vec<(String, u64)> = entity
                .id_infissi
                .iter()
                .filter(|x| unique.iter().any(|x1| x1.eq(&x.0)))
                .cloned()
                .collect();

            let infisso =
                StanzaConInfissi::new(entity.id_stanza, infissi, entity.id_edificio.clone());

            Self::insert(conn, infisso)?;
        }

        // Restituiamo l'entity aggiornato
        Ok(entity)
    }
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
    use crate::app_traits::{CreateTable, DaoTrait, Insert};
    use crate::dao::entity::{
        Edificio, Infisso, MaterialeInfisso, Stanza, TipoInfisso, VetroInfisso,
    };
    use crate::dao::utils::create_types_tables;
    use crate::dao::{
        EdificioDAO, InfissoDAO, MaterialeInfissoDAO, StanzaDAO, TipoInfissoDAO, VetroInfissoDAO,
    };
    use rusqlite::Connection;

    static ID_EDIFICIO: &str = "PR01-25";

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        create_types_tables(&conn).expect("Errore nella creazione delle tabelle dei tipi");

        let materiale = MaterialeInfisso::new("Legno", 1);
        MaterialeInfissoDAO::insert(&conn, materiale)
            .expect("Errore nella creazione del materiale");
        let materiale = MaterialeInfisso::new("PVC", 2);
        MaterialeInfissoDAO::insert(&conn, materiale)
            .expect("Errore nella creazione del materiale");

        let vetro = VetroInfisso::new("Singolo", 1);
        VetroInfissoDAO::insert(&conn, vetro).expect("Errore nella creazione del materiale");
        let vetro = VetroInfisso::new("Doppio", 2);
        VetroInfissoDAO::insert(&conn, vetro).expect("Errore nella creazione del materiale");

        let tipo_infisso = TipoInfisso {
            _id: 0,
            nome: "PORTA".to_string(),
        };
        TipoInfissoDAO::insert(&conn, tipo_infisso).expect("Errore nella creazione del tipo");
        let tipo_infisso = TipoInfisso {
            _id: 0,
            nome: "FINESTRA".to_string(),
        };
        TipoInfissoDAO::insert(&conn, tipo_infisso).expect("Errore nella creazione del tipo");

        EdificioDAO::create_table(&conn).unwrap();
        StanzaDAO::create_table(&conn).unwrap();
        InfissoDAO::create_table(&conn).unwrap();
        StanzaConInfissiDao::create_table(&conn).unwrap();

        let edificio = Edificio::new(ID_EDIFICIO, "00008545", "Via Pallone");
        EdificioDAO::insert(&conn, edificio.clone()).expect("Errore nella creazione dell'edificio");

        pretty_sqlite::print_table(&conn, &EdificioDAO::table_name()).unwrap();

        let stanza = Stanza::new("PR01-25", "T", "1250", "045", "Ufficio");
        StanzaDAO::insert(&conn, stanza).expect("Errore nella creazione della stanza");
        let stanza = Stanza::new("PR01-25", "T", "1250", "047", "Ufficio");
        StanzaDAO::insert(&conn, stanza).expect("Errore nella creazione della stanza");

        pretty_sqlite::print_table(&conn, &StanzaDAO::table_name()).unwrap();

        let infisso_a = Infisso::new("A", ID_EDIFICIO, "PORTA", 350, 450, "Legno", "Singolo");
        InfissoDAO::insert(&conn, infisso_a).expect("Errore nella creazione dell'infisso");

        let infisso_b = Infisso::new("B", ID_EDIFICIO, "FINESTRA", 350, 450, "PVC", "Doppio");
        InfissoDAO::insert(&conn, infisso_b).expect("Errore nella creazione dell'infisso");

        pretty_sqlite::print_table(&conn, &InfissoDAO::table_name()).unwrap();

        conn
    }

    #[test]
    fn create_table() {
        let conn = Connection::open_in_memory().unwrap();
        let res = StanzaConInfissiDao::create_table(&conn);
        assert!(res.is_ok());
    }

    #[test]
    fn insert_value() {
        let conn = setup();
        let stanza_con_infissi = StanzaConInfissi {
            id_stanza: 1,
            id_infissi: vec![("A".to_string(), 5), ("B".to_string(), 8)],
            id_edificio: ID_EDIFICIO.to_string(),
        };

        let res = StanzaConInfissiDao::insert(&conn, stanza_con_infissi.clone());
        match res {
            Ok(r) => {
                pretty_sqlite::print_table(&conn, &StanzaConInfissiDao::table_name()).unwrap();
                assert_eq!(r, stanza_con_infissi);
            }
            Err(e) => eprintln!("Errore: {}", e),
        }
    }

    #[test]
    fn get_stanza_con_infissi() {
        let conn = setup();

        let res = StanzaConInfissiDao::get_by_id(&conn, (1, ID_EDIFICIO.to_string()));
        match res {
            Ok(r) => {
                pretty_sqlite::print_table(&conn, &StanzaConInfissiDao::table_name()).unwrap();
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

        pretty_sqlite::print_table(&conn, &StanzaConInfissiDao::table_name()).unwrap();

        let res = StanzaConInfissiDao::get_all(&conn);
        match res {
            Ok(r) => {
                pretty_sqlite::print_table(&conn, &StanzaConInfissiDao::table_name()).unwrap();
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

        pretty_sqlite::print_table(&conn, &StanzaConInfissiDao::table_name()).unwrap();

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
                pretty_sqlite::print_table(&conn, &StanzaConInfissiDao::table_name()).unwrap();
                println!("{:?}", r);
            }
            Err(e) => {
                panic!("Errore: {}", e);
            }
        }
    }
}
