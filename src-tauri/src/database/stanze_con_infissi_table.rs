use crate::database::model::StanzaConInfisso;
use crate::database::Database;
use itertools::Itertools;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn get_stanza_con_infissi() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn insert_stanza_con_infissi(
    db: State<'_, Database>,
    new_value: StanzaConInfisso,
) -> Result<(), String> {
    db.with_transaction(|tx| {
        let mut ids_sorted = new_value.ids_infissi.clone();
        ids_sorted.sort();

        let conteggio_infissi: Vec<(String, i32)> = ids_sorted.into_iter()
            .chunk_by(|x| x.clone())
            .into_iter()
            .map(|(id, group)| (id, group.count() as i32))
            .collect();

        for (id_infisso, conteggio) in conteggio_infissi {
            tx.execute(
                "INSERT INTO STANZA_CON_INFISSI(ID_INFISSO, ID_STANZA, NUM_INFISSI) VALUES (?1, ?2, ?3)",
                params![new_value.id_stanza, id_infisso, conteggio],
            )
            .map_err(|e| format!("Errore nell'inserimento: {}", e))?;
        }
        Ok(())
    })
}
