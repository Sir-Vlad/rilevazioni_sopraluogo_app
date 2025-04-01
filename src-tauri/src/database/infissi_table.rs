use crate::database::model::Infisso;
use crate::database::Database;
use tauri::State;

#[tauri::command]
pub fn get_infissi(db: State<'_, Database>) -> Result<Vec<Infisso>, String> {
    let conn = db.conn.lock().unwrap();

    if let Some(conn) = conn.as_ref() {
        let mut stmt = conn
            .prepare("SELECT * FROM INFISSO")
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        let infissi: Result<Vec<Infisso>, rusqlite::Error> = stmt
            .query_map([], |row| {
                let id = row.get::<_, String>(0)?;
                let tipo = row.get::<_, String>(1)?;
                let altezza = row.get::<_, i32>(2)?;
                let larghezza = row.get::<_, i32>(3)?;
                let materiale = row.get::<_, String>(4)?;
                let vetro = row.get::<_, String>(5)?;

                Ok(Infisso::new(id, tipo, altezza, larghezza, materiale, vetro))
            })
            .expect("Errore nella lettura dei dati dal database")
            .collect();

        match infissi {
            Ok(infissi) => Ok(infissi),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database non inizializzato".to_string())
    }
}

#[tauri::command]
pub fn insert_infisso(db: State<'_, Database>, infisso: Infisso) -> Result<Infisso, String> {
    let conn = db.conn.lock().unwrap();
    if let Some(conn) = conn.as_ref() {
        conn.execute(
            "INSERT INTO INFISSI(ID, TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO) 
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
        Ok(infisso)
    } else {
        Err("Database non inizializzato".to_string())
    }
}
