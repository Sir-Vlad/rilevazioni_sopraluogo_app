use crate::dao::entity::Infisso;
use crate::dto::InfissoDto;
use rusqlite::Connection;

pub trait InfissoDao {
    fn get_all(conn: &Connection) -> Result<Vec<Infisso>, String>;
    fn insert(conn: &Connection, infisso: &InfissoDto) -> Result<Infisso, String>;
    fn update(conn: &Connection, infisso: &InfissoDto) -> Result<Infisso, String>;
}

pub struct InfissoDaoImpl;

impl InfissoDao for InfissoDaoImpl {
    fn get_all(conn: &Connection) -> Result<Vec<Infisso>, String> {
        let mut stmt = conn
            .prepare("SELECT * FROM INFISSO")
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        let infissi: Result<Vec<Infisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(Infisso {
                    id: row.get::<_, String>(0)?,
                    tipo: row.get::<_, String>(1)?,
                    altezza: row.get::<_, i32>(2)?,
                    larghezza: row.get::<_, i32>(3)?,
                    materiale: row.get::<_, String>(4)?,
                    vetro: row.get::<_, String>(5)?,
                })
            })
            .expect("Errore nella lettura dei dati dal database")
            .collect();

        match infissi {
            Ok(infissi) => Ok(infissi),
            Err(e) => Err(e.to_string()),
        }
    }

    fn insert(conn: &Connection, infisso: &InfissoDto) -> Result<Infisso, String> {
        conn.execute(
            "INSERT INTO INFISSO(ID, TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &infisso.id,
                &infisso.tipo,
                &infisso.altezza,
                &infisso.larghezza,
                &infisso.materiale,
                &infisso.vetro,
            ),
        )
        .map_err(|e| e.to_string())?;
        Ok(Infisso::from(infisso))
    }

    #[allow(dead_code, unused_variables)]
    fn update(conn: &Connection, infisso: &InfissoDto) -> Result<Infisso, String> {
        todo!()
    }
}
