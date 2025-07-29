#[cfg(test)]
mod test {
    use crate::models::{Edificio, Fotovoltaico, Infisso, Stanza, StanzaConInfissi, Utenza};
    use crate::schema::{edificio, fotovoltaico, infisso, stanza, stanza_con_infissi, utenze};
    use crate::tests::common::{setup, Result, Setup};
    use diesel::{
        r2d2::{ConnectionManager, PooledConnection},
        ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SqliteConnection,
    };

    #[test]
    fn test_full_migration_with_real_data() -> Result {
        let Setup {
            to,
            from,
            image: _image,
            temp_file: _tmp_file,
        } = setup()?;

        let mut conn_sq = from.get_sqlite_pool().get()?;
        let mut conn_pg = to.get_postgres_pool().get()?;

        // Crea e popola database SQLite con dati realistici
        setup_realistic_sqlite_data(&mut conn_sq)?;

        // Esegui la migrazione completa
        let migration = DatabaseMigrator::new(&from, &to);
        migration.migrate()?;

        // Verifica che i dati siano stati migrati correttamente
        verify_migration_results(&mut conn_pg)?;

        Ok(())
    }

    fn setup_realistic_sqlite_data(
        conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Result {
        // Inserisci dati realistici
        insert_realistic_data(conn)?;

        Ok(())
    }

    fn insert_realistic_data(
        conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Result {
        // Inserisci edifici
        diesel::sql_query(
            "INSERT INTO EDIFICIO (CHIAVE, FASCICOLO, INDIRIZZO, ANNO_COSTRUZIONE, ISOLAMENTO_TETTO, CAPPOTTO) VALUES
            ('EDI001', '12345', 'Via Roma 123', '1985', 1, 0),
            ('EDI002', '12346', 'Via Milano 456', '1990', 0, 1),
            ('EDI003', '12347', 'Via Napoli 789', '2000', 1, 1);"
        ).execute(conn)?;

        // Inserisci stanze
        diesel::sql_query(
            "INSERT INTO STANZA (EDIFICIO_ID, PIANO, ID_SPAZIO, COD_STANZA, DESTINAZIONE_USO, ALTEZZA, SPESSORE_MURO) VALUES
            ('EDI001', 'T', 'SP001', 'S001', 'Ufficio', 280, 20),
            ('EDI001', 'T', 'SP002', 'S002', 'Bagno', 260, 15),
            ('EDI001', '1', 'SP003', 'S003', 'Sala riunioni', 300, 25),
            ('EDI002', 'T', 'SP001', 'S001', 'Reception', 320, 20),
            ('EDI002', '1', 'SP002', 'S002', 'Ufficio', 280, 18);"
        ).execute(conn)?;

        // Inserisci infissi
        diesel::sql_query(
            "INSERT INTO INFISSO (ID, EDIFICIO_ID, TIPO, ALTEZZA, LARGHEZZA, MATERIALE, VETRO) VALUES
            ('A', 'EDI001', 'Porta', 2100, 800, 'Legno', 'Singolo'),
            ('B', 'EDI001', 'Finestra', 1200, 1000, 'PVC', 'Doppio'),
            ('C', 'EDI001', 'Finestra', 1100, 800, 'Alluminio', 'Triplo'),
            ('A', 'EDI002', 'Porta', 2000, 900, 'Legno', 'Singolo'),
            ('B', 'EDI002', 'Finestra', 1300, 1200, 'PVC', 'Doppio');"
        ).execute(conn)?;

        // Inserisci stanza con infissi
        diesel::sql_query(
            "INSERT INTO STANZA_CON_INFISSI (STANZA_ID, INFISSO_ID, EDIFICIO_ID, NUM_INFISSO) VALUES
            (1, 'A', 'EDI001', 1),
            (1, 'B', 'EDI001', 2),
            (2, 'C', 'EDI001', 1),
            (3, 'B', 'EDI001', 3),
            (4, 'A', 'EDI002', 1),
            (5, 'B', 'EDI002', 2);"
        ).execute(conn)?;

        // Inserisci utenze
        diesel::sql_query(
            "INSERT INTO UTENZE (EDIFICIO_ID, TIPO, COD_CONTATORE, INDIRIZZO_CONTATORE) VALUES
            ('EDI001', 'elettricità', 'ELE001234', 'Via Roma 123'),
            ('EDI001', 'acqua', 'IDR001234', 'Via Roma 123'),
            ('EDI001', 'riscaldamento', 'TER001234', 'Via Roma 123'),
            ('EDI002', 'elettricità', 'ELE005678', 'Via Milano 456'),
            ('EDI002', 'acqua', 'IDR005678', 'Via Milano 456');",
        )
        .execute(conn)?;

        // Inserisci fotovoltaico
        diesel::sql_query(
            "INSERT INTO FOTOVOLTAICO (EDIFICIO_ID, POTENZA, PROPRIETARIO) VALUES
            ('EDI001', 15.5, 'Mario Rossi'),
            ('EDI002', 22.3, 'Lucia Bianchi'),
            ('EDI003', 8.7, 'Giuseppe Verdi');",
        )
        .execute(conn)?;

        Ok(())
    }

    fn verify_migration_results(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result {
        // Verifica edifici
        let edifici: Vec<Edificio> = edificio::table.load(conn)?;
        assert_eq!(edifici.len(), 3);
        println!("Edifici migrati: {}", edifici.len());

        // Verifica stanze
        let stanze: Vec<Stanza> = stanza::table.load(conn)?;
        assert_eq!(stanze.len(), 5);
        println!("Stanze migrate: {}", stanze.len());

        // Verifica infissi
        let infissi: Vec<Infisso> = infisso::table.load(conn)?;
        assert_eq!(infissi.len(), 5);
        println!("Infissi migrati: {}", infissi.len());

        // Verifica stanza con infissi
        let stanza_infissi: Vec<StanzaConInfissi> = stanza_con_infissi::table.load(conn)?;
        assert_eq!(stanza_infissi.len(), 6);
        println!("Stanza con infissi migrati: {}", stanza_infissi.len());

        // Verifica utenze
        let utenze: Vec<Utenza> = utenze::table.load(conn)?;
        assert_eq!(utenze.len(), 5);
        println!("Utenze migrate: {}", utenze.len());

        // Verifica fotovoltaico
        let fotovoltaico: Vec<Fotovoltaico> = fotovoltaico::table.load(conn)?;
        assert_eq!(fotovoltaico.len(), 3);
        println!("Fotovoltaico migrato: {}", fotovoltaico.len());

        // Verifica che i mapping degli ID siano corretti
        let stanza_prima = stanze
            .iter()
            .find(|s| s.cod_stanza == "S001" && s.edificio_id.trim() == "EDI001")
            .unwrap();
        let infissi_stanza = stanza_con_infissi::table
            .filter(stanza_con_infissi::stanza_id.eq(stanza_prima.id))
            .load::<StanzaConInfissi>(conn)?;
        assert!(!infissi_stanza.is_empty());
        println!("Mapping ID verificato correttamente");

        Ok(())
    }
}
