use std::fmt::Debug;

use chrono::NaiveDateTime;
use diesel::{
    AsChangeset, Associations, Identifiable, Insertable, Queryable, QueryableByName, Selectable,
    sql_types::{Integer, Nullable, SmallInt, Text},
};
use serde::{Deserialize, Serialize};

use crate::schema::{
    annotazione_edificio, annotazione_infisso, annotazione_stanza, climatizzazione, edificio,
    fotovoltaico, illuminazione, infisso, materiale_infisso, stanza, stanza_con_infissi,
    tipo_infisso, utenze, vetro_infisso,
};

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq)]
#[diesel(table_name = illuminazione)]
pub struct Illuminazione {
    pub lampadina: String,
    pub eff_energetica: i16,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = climatizzazione)]
#[diesel(primary_key(nome))]
pub struct Climatizzazione {
    pub nome: String,
    pub eff_energetica: i16,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = materiale_infisso)]
pub struct MaterialeInfisso {
    pub materiale: String,
    pub eff_energetica: i16,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = vetro_infisso)]
pub struct VetroInfisso {
    pub vetro: String,
    pub eff_energetica: i16,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = tipo_infisso)]
pub struct TipoInfisso {
    pub nome: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = edificio)]
#[diesel(primary_key(chiave))]
pub struct Edificio {
    pub chiave: String,
    pub fascicolo: i32,
    pub indirizzo: String,
    pub anno_costruzione: Option<i32>,
    pub anno_riqualificazione: Option<i32>,
    pub note_riqualificazione: Option<String>,
    pub isolamento_tetto: bool,
    pub cappotto: bool,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = edificio)]
#[cfg_attr(feature = "default", derive(Clone))]
pub struct NewEdificio {
    pub chiave: String,
    pub fascicolo: i32,
    pub indirizzo: String,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = edificio)]
pub struct UpdateEdificio {
    pub anno_costruzione: Option<i32>,
    pub anno_riqualificazione: Option<i32>,
    pub note_riqualificazione: Option<String>,
    pub isolamento_tetto: Option<bool>,
    pub cappotto: Option<bool>,
}

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Edificio))]
#[diesel(table_name = stanza)]
pub struct Stanza {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub edificio_id: String,
    #[diesel(sql_type = Text)]
    pub piano: String,
    #[diesel(sql_type = Text)]
    pub id_spazio: String,
    #[diesel(sql_type = Text)]
    pub cod_stanza: String,
    #[diesel(sql_type = Text)]
    pub destinazione_uso: String,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub altezza: Option<i16>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub spessore_muro: Option<i16>,
    #[diesel(sql_type = Nullable<Text>)]
    pub riscaldamento: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub raffrescamento: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub illuminazione: Option<String>,
}

impl AsRef<Stanza> for Stanza {
    fn as_ref(&self) -> &Self { self }
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = stanza)]
pub struct NewStanza {
    pub edificio_id: String,
    pub piano: String,
    pub id_spazio: String,
    pub cod_stanza: String,
    pub destinazione_uso: String,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = stanza)]
pub struct UpdateStanza {
    pub altezza: Option<i16>,
    pub spessore_muro: Option<i16>,
    pub riscaldamento: Option<String>,
    pub raffrescamento: Option<String>,
    pub illuminazione: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = infisso)]
#[diesel(primary_key(id, edificio_id))]
pub struct Infisso {
    pub id: String,
    pub edificio_id: String,
    pub tipo: String,
    pub altezza: i16,
    pub larghezza: i16,
    pub materiale: String,
    pub vetro: String,
    pub mq: f32,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = infisso)]
pub struct NewInfisso {
    pub id: String,
    pub edificio_id: String,
    pub tipo: String,
    pub altezza: i16,
    pub larghezza: i16,
    pub materiale: String,
    pub vetro: String,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = infisso)]
pub struct UpdateInfisso {
    pub altezza: Option<i16>,
    pub larghezza: Option<i16>,
    pub materiale: Option<String>,
    pub vetro: Option<String>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Insertable, Debug, PartialEq, Clone,
)]
#[diesel(table_name = stanza_con_infissi)]
#[diesel(primary_key(infisso_id, edificio_id, stanza_id))]
#[diesel(belongs_to(Stanza, foreign_key = stanza_id))]
#[cfg_attr(test, derive(Deserialize))]
pub struct StanzaConInfissi {
    pub infisso_id: String,
    pub edificio_id: String,
    pub stanza_id: i32,
    pub num_infisso: i32,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = stanza_con_infissi)]
pub struct UpdateStanzaConInfissi {
    pub num_infisso: i32,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = fotovoltaico)]
pub struct Fotovoltaico {
    pub id: i32,
    pub edificio_id: String,
    pub potenza: f32,
    pub proprietario: String,
}

#[cfg_attr(feature = "test-models", derive(Clone))]
#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = fotovoltaico)]
pub struct NewFotovoltaico {
    pub edificio_id: String,
    pub potenza: f32,
    pub proprietario: String,
}

#[cfg_attr(feature = "test-models", derive(Clone))]
#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = fotovoltaico)]
pub struct UpdateFotovoltaico {
    pub potenza: Option<f32>,
    pub proprietario: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = utenze)]
pub struct Utenza {
    pub id: i32,
    pub edificio_id: String,
    pub tipo: TipoUtenza,
    pub cod_contatore: String,
    pub indirizzo_contatore: Option<String>,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = utenze)]
pub struct NewUtenza {
    pub edificio_id: String,
    pub tipo: TipoUtenza,
    pub cod_contatore: String,
    pub indirizzo_contatore: Option<String>,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = utenze)]
pub struct UpdateUtenza {
    pub tipo: Option<TipoUtenza>,
    pub cod_contatore: Option<String>,
    pub indirizzo_contatore: Option<String>,
}

#[derive(Serialize, Deserialize, diesel_derive_enum::DbEnum, Debug, PartialEq, Clone)]
#[ExistingTypePath = "crate::schema::sql_types::TipoUtenza"]
pub enum TipoUtenza {
    Acqua,
    #[db_rename = "elettricità"]
    Elettricità,
    Riscaldamento,
}

impl From<&str> for TipoUtenza {
    fn from(value: &str) -> Self {
        match value {
            "idrica" | "acqua" => TipoUtenza::Acqua,
            "termica" | "riscaldamento" => TipoUtenza::Riscaldamento,
            "elettrica" | "elettricità" => TipoUtenza::Elettricità,
            _ => panic!("TipoUtenza non riconosciuto: {value}"),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = annotazione_edificio)]
pub struct AnnotazioneEdificio {
    pub id: i32,
    pub edificio_id: String,
    pub content: String,
    pub data: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = annotazione_edificio)]
pub struct NewAnnotazioneEdificio {
    pub edificio_id: String,
    pub content: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = annotazione_stanza)]
pub struct AnnotazioneStanza {
    pub id: i32,
    pub stanza_id: i32,
    pub content: String,
    pub data: NaiveDateTime,
}

#[cfg_attr(feature = "test-models", derive(Clone))]
#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = annotazione_stanza)]
pub struct NewAnnotazioneStanza {
    pub stanza_id: i32,
    pub content: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = annotazione_infisso)]
pub struct AnnotazioneInfisso {
    pub id: i32,
    pub infisso_id: String,
    pub edificio_id: String,
    pub content: String,
    pub data: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = annotazione_infisso)]
pub struct NewAnnotazioneInfisso {
    pub infisso_id: String,
    pub edificio_id: String,
    pub content: String,
}

#[derive(Queryable, QueryableByName, Debug)]
pub struct DatiStanza {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub fascicolo: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub chiave: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub piano: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub id_spazio: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub cod_stanza: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub destinazione_uso: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::SmallInt>)]
    pub altezza: Option<i16>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::SmallInt>)]
    pub spessore_muro: Option<i16>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub riscaldamento: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub raffrescamento: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub illuminazione: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Double)]
    pub mq_infissi: f64,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub materiale: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub vetro: Option<String>,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use diesel::{PgConnection, RunQueryDsl, dsl::sql, prelude::*, sql_types::Integer};
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
    use testcontainers::{Container, runners::SyncRunner};
    use testcontainers_modules::postgres::Postgres;

    use super::*;

    fn setup_postgresql_database() -> Result<(PgConnection, Container<Postgres>), Box<dyn Error>> {
        let image = Postgres::default().start()?;

        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            image.get_host_port_ipv4(5432)?
        );

        let mut conn = PgConnection::establish(connection_string)?;
        try_run_migrations_with_retry(&mut conn, 10)?;
        Ok((conn, image))
    }

    fn try_run_migrations_with_retry(
        conn: &mut PgConnection,
        retries: u32,
    ) -> Result<(), Box<dyn Error>> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

        for attempt in 0..retries {
            match conn.run_pending_migrations(MIGRATIONS) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    println!(
                        "Tentativo {attempt}: sintassi non accettata o DB non pronto. Attendo..."
                    );
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }

        Err("Migrazioni fallite dopo i tentativi".into())
    }

    #[test]
    fn test_database_connection() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        #[derive(QueryableByName, Debug)]
        struct TestResult {
            #[diesel(sql_type = diesel::sql_types::Integer)]
            test_col: i32,
        }

        let result = diesel::sql_query("SELECT 1 as test_col").load::<TestResult>(&mut conn)?;

        assert!(!result.is_empty());
        println!("Database connection test passed!");

        Ok(())
    }

    #[test]
    fn test_illuminazione() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        match illuminazione::table.load::<Illuminazione>(&mut conn) {
            Ok(res) => {
                println!("{res:?}");
                assert!(!res.is_empty());
                Ok(())
            }
            Err(err) => panic!("Error: {err:?}"),
        }
    }

    #[test]
    fn test_climatizzazione() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        match climatizzazione::table.load::<Climatizzazione>(&mut conn) {
            Ok(res) => {
                println!("{res:?}");
                assert!(!res.is_empty());
                Ok(())
            }
            Err(err) => panic!("Error: {err:?}"),
        }
    }

    #[test]
    fn test_materiale_infisso() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        match materiale_infisso::table.load::<MaterialeInfisso>(&mut conn) {
            Ok(res) => {
                println!("{res:?}");
                assert!(!res.is_empty());
                Ok(())
            }
            Err(err) => panic!("Error: {err:?}"),
        }
    }

    #[test]
    fn test_vetro_infisso() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        match vetro_infisso::table.load::<VetroInfisso>(&mut conn) {
            Ok(res) => {
                println!("{res:?}");
                assert!(!res.is_empty());
                Ok(())
            }
            Err(err) => panic!("Error: {err:?}"),
        }
    }

    fn insert_edificio_standard(conn: &mut PgConnection) -> Result<Edificio, Box<dyn Error>> {
        let new_values = NewEdificio {
            chiave: "1234567".to_string(),
            fascicolo: 1,
            indirizzo: "Via Roma 1".to_string(),
        };
        Ok(diesel::insert_into(edificio::table)
            .values(&new_values)
            .get_result(conn)?)
    }

    #[test]
    fn test_insert_edificio() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;
        let edificio = insert_edificio_standard(&mut conn)?;
        println!("{edificio:#?}");
        Ok(())
    }

    #[test]
    fn test_update_edificio() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let edificio = insert_edificio_standard(&mut conn)?;

        let mut update_value = UpdateEdificio {
            anno_costruzione: Some(1990),
            anno_riqualificazione: None,
            note_riqualificazione: None,
            isolamento_tetto: None,
            cappotto: None,
        };

        let result: Edificio = diesel::update(edificio::table.find(&edificio.chiave))
            .set(&update_value)
            .get_result(&mut conn)?;
        assert_eq!(result.anno_costruzione, Some(1990));

        update_value.anno_costruzione = Some(1995);
        update_value.cappotto = Some(true);

        let result: Edificio = diesel::update(edificio::table.find(&edificio.chiave))
            .set(&update_value)
            .get_result(&mut conn)?;
        assert_eq!(result.anno_costruzione, Some(1995));
        assert!(result.cappotto);

        update_value.anno_costruzione = None;
        update_value.cappotto = None;
        update_value.anno_riqualificazione = Some(2004);

        let result: Edificio = diesel::update(edificio::table.find(&edificio.chiave))
            .set(&update_value)
            .get_result(&mut conn)?;
        assert_eq!(result.anno_riqualificazione, Some(2004));
        assert_ne!(result.anno_costruzione, None);

        Ok(())
    }

    fn insert_stanza_standard(
        conn: &mut PgConnection,
        edificio_id: &str,
    ) -> Result<Stanza, Box<dyn Error>> {
        let new_values = NewStanza {
            edificio_id: edificio_id.to_string(),
            piano: "T".to_string(),
            id_spazio: "145236".to_string(),
            cod_stanza: "001".to_string(),
            destinazione_uso: "Ufficio".to_string(),
        };

        Ok(diesel::insert_into(stanza::table)
            .values(&new_values)
            .get_result(conn)?)
    }

    #[test]
    fn test_insert_stanza() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let edificio = insert_edificio_standard(&mut conn)?;
        let inserted = insert_stanza_standard(&mut conn, edificio.chiave.as_str())?;
        println!("{inserted:#?}");
        Ok(())
    }

    #[test]
    fn test_update_stanza() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let edificio = insert_edificio_standard(&mut conn)?;
        let stanza = insert_stanza_standard(&mut conn, edificio.chiave.as_str())?;

        let update_stanza = UpdateStanza {
            altezza: Some(120),
            spessore_muro: None,
            riscaldamento: Some("Radiatori".to_string()),
            raffrescamento: None,
            illuminazione: None,
        };

        let updated: Stanza = diesel::update(stanza::table.find(stanza.id))
            .set(&update_stanza)
            .get_result(&mut conn)?;
        assert_eq!(updated.altezza, Some(120));
        assert_eq!(updated.riscaldamento, Some("Radiatori".to_string()));

        Ok(())
    }

    fn insert_infisso_standard(
        conn: &mut PgConnection,
        edificio_id: &str,
    ) -> Result<Infisso, Box<dyn Error>> {
        let insert_infisso = NewInfisso {
            id: "A".to_string(),
            edificio_id: edificio_id.to_string(),
            tipo: "Porta".to_string(),
            altezza: 120,
            larghezza: 150,
            materiale: "Legno".to_string(),
            vetro: "Singolo".to_string(),
        };

        Ok(diesel::insert_into(infisso::table)
            .values(&insert_infisso)
            .get_result(conn)?)
    }

    #[test]
    fn test_insert_infisso() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let edificio = insert_edificio_standard(&mut conn)?;
        let inserted = insert_infisso_standard(&mut conn, edificio.chiave.as_str())?;

        println!("{inserted:#?}");

        Ok(())
    }

    #[test]
    fn test_update_infisso() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let edificio = insert_edificio_standard(&mut conn)?;
        let inserted = insert_infisso_standard(&mut conn, edificio.chiave.as_str())?;

        let update_infisso = UpdateInfisso {
            altezza: Some(135),
            larghezza: None,
            materiale: Some("PVC".to_string()),
            vetro: None,
        };

        let updated_infisso: Infisso =
            diesel::update(infisso::table.find((inserted.id.trim(), inserted.edificio_id)))
                .set(&update_infisso)
                .get_result(&mut conn)?;
        assert_eq!(updated_infisso.altezza, 135);
        assert_eq!(updated_infisso.larghezza, inserted.larghezza);
        assert_eq!(updated_infisso.materiale, "PVC");
        assert_eq!(updated_infisso.vetro, inserted.vetro);

        Ok(())
    }

    fn insert_stanza_con_infissi(
        conn: &mut PgConnection,
    ) -> Result<Vec<StanzaConInfissi>, Box<dyn Error>> {
        let edificio = insert_edificio_standard(conn)?;
        let stanza = insert_stanza_standard(conn, edificio.chiave.as_str())?;
        let infisso_a = insert_infisso_standard(conn, edificio.chiave.as_str())?;

        let infisso_b = NewInfisso {
            id: "B".to_string(),
            edificio_id: edificio.chiave,
            tipo: "Porta".to_string(),
            altezza: 120,
            larghezza: 150,
            materiale: "Legno".to_string(),
            vetro: "Singolo".to_string(),
        };

        let infisso_b: Infisso = diesel::insert_into(infisso::table)
            .values(&infisso_b)
            .get_result(conn)?;

        let stanza_con_infissi = vec![
            StanzaConInfissi {
                infisso_id: infisso_a.id.clone(),
                edificio_id: infisso_a.edificio_id.clone(),
                stanza_id: stanza.id,
                num_infisso: 10,
            },
            StanzaConInfissi {
                infisso_id: infisso_b.id.clone(),
                edificio_id: infisso_b.edificio_id.clone(),
                stanza_id: stanza.id,
                num_infisso: 10,
            },
        ];
        Ok(diesel::insert_into(stanza_con_infissi::table)
            .values(&stanza_con_infissi)
            .get_results(conn)?)
    }

    #[test]
    fn test_insert_stanza_con_infissi() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let results = insert_stanza_con_infissi(&mut conn)?;
        println!("{results:#?}");

        Ok(())
    }

    #[test]
    fn update_stanza_con_infissi() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let results = insert_stanza_con_infissi(&mut conn)?;

        let update = StanzaConInfissi {
            infisso_id: "B".to_string(),
            edificio_id: results[0].edificio_id.clone(),
            stanza_id: results[0].stanza_id,
            num_infisso: 10,
        };

        let updated: StanzaConInfissi = diesel::update(stanza_con_infissi::table.find((
            update.infisso_id,
            update.edificio_id,
            update.stanza_id,
        )))
        .set(stanza_con_infissi::num_infisso.eq(
            sql::<Integer>("COALESCE(num_infisso, 0) + ").bind::<Integer, _>(update.num_infisso),
        ))
        .get_result(&mut conn)?;
        assert_eq!(updated.num_infisso, 20);
        println!("{updated:#?}");

        Ok(())
    }

    #[test]
    fn test_insert_annotazione_edificio() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let edificio = insert_edificio_standard(&mut conn)?;

        let insert_annotazione = NewAnnotazioneEdificio {
            edificio_id: edificio.chiave,
            content: "Sono una nuova annotazione".to_string(),
        };

        let inserted: AnnotazioneEdificio = diesel::insert_into(annotazione_edificio::table)
            .values(&insert_annotazione)
            .get_result(&mut conn)?;
        println!("{inserted:#?}");
        assert_eq!(inserted.content, "Sono una nuova annotazione");

        Ok(())
    }

    #[test]
    fn test_insert_annotazione_stanza() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let edificio = insert_edificio_standard(&mut conn)?;
        let stanza = insert_stanza_standard(&mut conn, edificio.chiave.as_str())?;

        let insert_annotazione = NewAnnotazioneStanza {
            stanza_id: stanza.id,
            content: "Sono una nuova annotazione".to_string(),
        };

        let inserted: AnnotazioneStanza = diesel::insert_into(annotazione_stanza::table)
            .values(&insert_annotazione)
            .get_result(&mut conn)?;
        println!("{inserted:#?}");
        assert_eq!(inserted.content, "Sono una nuova annotazione");

        Ok(())
    }

    #[test]
    fn test_insert_annotazione_infisso() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let edificio = insert_edificio_standard(&mut conn)?;
        let infisso = insert_infisso_standard(&mut conn, edificio.chiave.as_str())?;

        let insert_annotazione = NewAnnotazioneInfisso {
            infisso_id: infisso.id,
            edificio_id: infisso.edificio_id,
            content: "Sono una nuova annotazione".to_string(),
        };

        let inserted: AnnotazioneInfisso = diesel::insert_into(annotazione_infisso::table)
            .values(&insert_annotazione)
            .get_result(&mut conn)?;
        println!("{inserted:#?}");
        assert_eq!(inserted.content, "Sono una nuova annotazione");

        Ok(())
    }

    fn insert_fotovoltaico_standard(
        conn: &mut PgConnection,
    ) -> Result<Fotovoltaico, Box<dyn Error>> {
        let edificio = insert_edificio_standard(conn)?;

        let insert_entity = NewFotovoltaico {
            edificio_id: edificio.chiave,
            potenza: 85.0,
            proprietario: "Ugo Ugolini".to_string(),
        };

        Ok(diesel::insert_into(fotovoltaico::table)
            .values(&insert_entity)
            .get_result(conn)?)
    }

    #[test]
    fn test_insert_fotovoltaico() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let inserted = insert_fotovoltaico_standard(&mut conn)?;

        println!("{inserted:#?}");
        assert_eq!(inserted.id, 1);
        assert_eq!(inserted.potenza, 85.0);
        Ok(())
    }

    #[test]
    fn test_update_fotovoltaico() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let inserted = insert_fotovoltaico_standard(&mut conn)?;

        let update_entity = UpdateFotovoltaico {
            potenza: Some(100.0),
            proprietario: None,
        };

        let updated: Fotovoltaico = diesel::update(fotovoltaico::table.find(inserted.id))
            .set(&update_entity)
            .get_result(&mut conn)?;
        assert_eq!(updated.potenza, 100.0);

        Ok(())
    }

    fn insert_utenza_standard(conn: &mut PgConnection) -> Result<Utenza, Box<dyn Error>> {
        let edificio = insert_edificio_standard(conn)?;

        let insert_entity = NewUtenza {
            edificio_id: edificio.chiave,
            tipo: TipoUtenza::Acqua,
            cod_contatore: "785452215588".to_string(),
            indirizzo_contatore: Some("Via Roma 1".to_string()),
        };

        Ok(diesel::insert_into(utenze::table)
            .values(&insert_entity)
            .get_result(conn)?)
    }

    #[test]
    fn test_insert_utenza() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let inserted = insert_utenza_standard(&mut conn)?;

        println!("{inserted:#?}");
        assert_eq!(inserted.id, 1);
        assert_eq!(inserted.tipo, TipoUtenza::Acqua);
        Ok(())
    }

    #[test]
    fn test_update_utenza() -> Result<(), Box<dyn Error>> {
        let (mut conn, _container) = setup_postgresql_database()?;

        let inserted = insert_utenza_standard(&mut conn)?;

        let update_entity = UpdateUtenza {
            tipo: Some(TipoUtenza::Elettricità),
            cod_contatore: Some("456789".to_string()),
            indirizzo_contatore: None,
        };

        let updated: Utenza = diesel::update(utenze::table.find(inserted.id))
            .set(&update_entity)
            .get_result(&mut conn)?;
        assert_eq!(updated.tipo, TipoUtenza::Elettricità);
        assert_eq!(updated.cod_contatore, "456789");

        Ok(())
    }
}
