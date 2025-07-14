use crate::schema::{
    annotazione_edificio, annotazione_infisso, annotazione_stanza, climatizzazione, edificio,
    fotovoltaico, illuminazione, infisso, materiale_infisso, stanza, stanza_con_infissi,
    tipo_infisso, utenze, vetro_infisso,
};
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use std::fmt::Debug;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = illuminazione)]
pub struct Illuminazione {
    lampadina: String,
    eff_energetica: i16,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = climatizzazione)]
#[diesel(primary_key(nome))]
pub struct Climatizzazione {
    nome: String,
    eff_energetica: i16,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = materiale_infisso)]
pub struct MaterialeInfisso {
    materiale: String,
    eff_energetica: i16,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = vetro_infisso)]
pub struct VetroInfisso {
    vetro: String,
    eff_energetica: i16,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = tipo_infisso)]
pub struct TipoInfisso {
    nome: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = edificio)]
#[diesel(primary_key(chiave))]
pub struct Edificio {
    pub(crate) chiave: String,
    pub(crate) fascicolo: i32,
    pub(crate) indirizzo: String,
    pub(crate) anno_costruzione: Option<i32>,
    pub(crate) anno_riqualificazione: Option<i32>,
    pub(crate) note_riqualificazione: Option<String>,
    pub(crate) isolamento_tetto: bool,
    pub(crate) cappotto: bool,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = edificio)]
pub struct NewEdificio<'a> {
    pub(crate) chiave: &'a str,
    pub(crate) fascicolo: i32,
    pub(crate) indirizzo: &'a str,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = edificio)]
pub struct UpdateEdificio<'a> {
    anno_costruzione: Option<i32>,
    anno_riqualificazione: Option<i32>,
    note_riqualificazione: Option<&'a str>,
    isolamento_tetto: Option<bool>,
    cappotto: Option<bool>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Edificio))]
#[diesel(table_name = stanza)]
pub struct Stanza {
    pub(crate) id: i32,
    pub(crate) edificio_id: String,
    pub(crate) piano: String,
    pub(crate) id_spazio: String,
    pub(crate) cod_stanza: String,
    pub(crate) destinazione_uso: String,
    pub(crate) altezza: Option<i16>,
    pub(crate) spessore_muro: Option<i16>,
    pub(crate) riscaldamento: Option<String>,
    pub(crate) raffrescamento: Option<String>,
    pub(crate) illuminazione: Option<String>,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = stanza)]
pub struct NewStanza<'a> {
    edificio_id: &'a str,
    piano: &'a str,
    id_spazio: &'a str,
    cod_stanza: &'a str,
    destinazione_uso: &'a str,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = stanza)]
pub struct UpdateStanza<'a> {
    altezza: Option<i16>,
    spessore_muro: Option<i16>,
    riscaldamento: Option<&'a str>,
    raffrescamento: Option<&'a str>,
    illuminazione: Option<&'a str>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = infisso)]
#[diesel(primary_key(id, edificio_id))]
pub struct Infisso {
    pub(crate) id: String,
    pub(crate) edificio_id: String,
    pub(crate) tipo: String,
    pub(crate) altezza: i16,
    pub(crate) larghezza: i16,
    pub(crate) materiale: String,
    pub(crate) vetro: String,
    pub(crate) mq: f32,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = infisso)]
pub struct NewInfisso<'a> {
    pub(crate) id: &'a str,
    pub(crate) edificio_id: &'a str,
    pub(crate) tipo: &'a str,
    pub(crate) altezza: i16,
    pub(crate) larghezza: i16,
    pub(crate) materiale: &'a str,
    pub(crate) vetro: &'a str,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = infisso)]
pub struct UpdateInfisso<'a> {
    altezza: Option<i16>,
    larghezza: Option<i16>,
    materiale: Option<&'a str>,
    vetro: Option<&'a str>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Insertable, Debug, PartialEq)]
#[diesel(table_name = stanza_con_infissi)]
#[diesel(primary_key(infisso_id, edificio_id, stanza_id))]
#[diesel(belongs_to(Stanza, foreign_key = stanza_id))]
pub struct StanzaConInfissi {
    pub(crate) infisso_id: String,
    pub(crate) edificio_id: String,
    pub(crate) stanza_id: i32,
    pub(crate) num_infisso: i32,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = fotovoltaico)]
pub struct Fotovoltaico {
    id: i32,
    pub(crate) edificio_id: String,
    pub(crate) potenza: f32,
    pub(crate) proprietario: String,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = fotovoltaico)]
pub struct NewFotovoltaico<'a> {
    pub(crate) edificio_id: &'a str,
    pub(crate) potenza: f32,
    pub(crate) proprietario: &'a str,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = fotovoltaico)]
pub struct UpdateFotovoltaico<'a> {
    potenza: Option<f32>,
    proprietario: Option<&'a str>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = utenze)]
pub struct Utenza {
    id: i32,
    pub(crate) edificio_id: String,
    pub(crate) tipo: TipoUtenza,
    pub(crate) cod_contatore: String,
    pub(crate) indirizzo_contatore: Option<String>,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = utenze)]
pub struct NewUtenza<'a> {
    pub(crate) edificio_id: &'a str,
    pub(crate) tipo: TipoUtenza,
    pub(crate) cod_contatore: &'a str,
    pub(crate) indirizzo_contatore: Option<&'a str>,
}

#[derive(AsChangeset, Debug, PartialEq)]
#[diesel(table_name = utenze)]
pub struct UpdateUtenza<'a> {
    tipo: Option<TipoUtenza>,
    cod_contatore: Option<&'a str>,
    indirizzo_contatore: Option<&'a str>,
}

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq, Clone)]
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
            "idrica" => TipoUtenza::Acqua,
            "termica" => TipoUtenza::Riscaldamento,
            "elettrica" => TipoUtenza::Elettricità,
            _ => panic!("TipoUtenza non riconosciuto: {}", value),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = annotazione_edificio)]
pub struct AnnotazioneEdificio {
    id: i32,
    edificio_id: String,
    content: String,
    data: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = annotazione_edificio)]
pub struct NewAnnotazioneEdificio<'a> {
    edificio_id: &'a str,
    content: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = annotazione_stanza)]
pub struct AnnotazioneStanza {
    id: i32,
    stanza_id: i32,
    content: String,
    data: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = annotazione_stanza)]
pub struct NewAnnotazioneStanza<'a> {
    stanza_id: i32,
    content: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = annotazione_infisso)]
pub struct AnnotazioneInfisso {
    id: i32,
    infisso_id: String,
    edificio_id: String,
    content: String,
    data: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = annotazione_infisso)]
pub struct NewAnnotazioneInfisso<'a> {
    infisso_id: &'a str,
    edificio_id: &'a str,
    content: &'a str,
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel::{dsl::sql, prelude::*, sql_types::Integer, PgConnection, RunQueryDsl};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use std::error::Error;
    use testcontainers::{runners::SyncRunner, Container};
    use testcontainers_modules::postgres::Postgres;

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
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

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
            chiave: "1234567",
            fascicolo: 1,
            indirizzo: "Via Roma 1",
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
            edificio_id,
            piano: "T",
            id_spazio: "145236",
            cod_stanza: "001",
            destinazione_uso: "Ufficio",
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
            riscaldamento: Some("Radiatori"),
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
            id: "A",
            edificio_id,
            tipo: "Porta",
            altezza: 120,
            larghezza: 150,
            materiale: "Legno",
            vetro: "Singolo",
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
            materiale: Some("PVC"),
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
            id: "B",
            edificio_id: edificio.chiave.as_str(),
            tipo: "Porta",
            altezza: 120,
            larghezza: 150,
            materiale: "Legno",
            vetro: "Singolo",
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
            edificio_id: edificio.chiave.as_str(),
            content: "Sono una nuova annotazione",
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
            content: "Sono una nuova annotazione",
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
            infisso_id: infisso.id.as_str(),
            edificio_id: infisso.edificio_id.as_str(),
            content: "Sono una nuova annotazione",
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
            edificio_id: edificio.chiave.as_str(),
            potenza: 85.0,
            proprietario: "Ugo Ugolini",
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
            edificio_id: edificio.chiave.as_str(),
            tipo: TipoUtenza::Acqua,
            cod_contatore: "785452215588",
            indirizzo_contatore: Some("Via Roma 1"),
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
            cod_contatore: Some("456789"),
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
