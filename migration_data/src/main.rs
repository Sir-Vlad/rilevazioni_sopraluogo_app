use crate::migrations::ConnectionPool;
use diesel::{PgConnection, SqliteConnection};
use dirs_next::document_dir;
use dotenvy::dotenv;
use std::fs::File;
use std::process::Stdio;
use std::{env, fs};
use tracing::{error, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod errors;
mod migrations;

const DB_PATH: &str = "Dati_Sopralluogo";

fn main() {
    /*
        1. Controllare se esiste la cartella `Dati_Sopralluogo`, se non esiste exit
        2. Eseguire il file `migrate_sqlite.sql` su tutti i file sqlite
        3. Connettersi a postgres
        4. Ottenere tutti i file sqlite presenti e uno alla volta eseguire la migrazione

        Migrazione
        1. Aprire la connessione con il file sqlite
        2. Avviare la funzione di migrazione
        3. Chiudere la connessione
    */
    dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args: Vec<String> = env::args().collect();
    let postgres_url = if args.len() > 1 {
        args[1].clone()
    } else {
        warn!("Using default postgres url");
        "postgresql://app_user:app_password@localhost:5432/app_development".to_string()
    };



    let folder_document = document_dir();
    let db_path = if let Some(folder_document) = folder_document {
        let path = folder_document.join(DB_PATH);
        if path.exists() {
            path
        } else {
            error!("Folder not found");
            std::process::exit(1);
        }
    } else {
        error!("Document folder not found");
        std::process::exit(1);
    };

    let files = fs::read_dir(&db_path).unwrap();
    let mut database_files = Vec::new();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        if is_valid_sqlite_file(&path) {
            database_files.push(path);
        }
    }

    let folder_migrations = db_path.join("migrations");
    if !folder_migrations.exists() {
        fs::create_dir(&folder_migrations).unwrap_or_else(|e| {
            error!("Errore creazione cartella migrations: {e}");
            std::process::exit(1);
        });
    }

    let mut failed_migrations = Vec::new();
    let mut success_migrations = Vec::new();
    for path in database_files {
        let new_path = folder_migrations.join(path.file_name().unwrap());
        match fs::copy(&path, &new_path) {
            Ok(_) => {}
            Err(e) => {
                warn!("Errore copia file {path:?}: {e}");
                continue;
            }
        }

        let name_db = new_path.file_name().unwrap().to_str().unwrap();
        match run_sql_script(&new_path) {
            Ok(_) => {
                info!("Modifica {name_db} riuscita con successo.");
                success_migrations.push(new_path);
            }
            Err(e) => {
                warn!("Errore in {name_db}: {e}");
                fs::remove_file(&new_path).unwrap();
                failed_migrations.push(name_db.to_string());
                continue;
            }
        }
    }

    warn!("Failed migrations: {failed_migrations:#?}");

    let postgres_conn = diesel::r2d2::ConnectionManager::<PgConnection>::new(postgres_url);
    let postgres_pool = diesel::r2d2::Pool::builder().build(postgres_conn).unwrap();
    let postgres_pool = ConnectionPool::Postgres(postgres_pool);

    for sqlite_path in success_migrations {
        let database_url = sqlite_path.as_path().to_str().unwrap();
        let sqlite_conn = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
        let sqlite_pool = diesel::r2d2::Pool::builder().build(sqlite_conn).unwrap();
        let sqlite_pool = ConnectionPool::Sqlite(sqlite_pool);

        let db_migrator = migrations::DatabaseMigrator::new(&sqlite_pool, &postgres_pool);
        match db_migrator.migrate() {
            Ok(_) => {
                info!("Migrazione {sqlite_path:?}: OK");
            }
            Err(e) => {
                warn!("Errore migrazione {sqlite_path:?}: {e}");

                println!("Vuoi continuare: ");
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        if input.trim().to_lowercase().eq("y") { continue; } else { std::process::exit(0); }
                    },
                    Err(_) => {
                        warn!("Errore nel leggere l'input");
                        continue;
                    }
                };
            }
        }
    }
}

fn is_valid_sqlite_file(path: &std::path::Path) -> bool {
    if !path.is_file() {
        return false;
    }

    let extension = match path.extension() {
        None => return false,
        Some(ext) => ext,
    };
    if extension != "db" {
        return false;
    }

    let file_name = match path.file_name() {
        None => return false,
        Some(file_name) => file_name,
    };
    let file_name_str = match file_name.to_str() {
        None => return false,
        Some(file_name) => file_name,
    };

    if file_name_str.starts_with(".") || file_name_str.contains("backup") {
        return false;
    }

    let split_file_name = file_name_str.split(".").collect::<Vec<&str>>();
    split_file_name
        .first()
        .is_some_and(|s| s.parse::<u64>().is_ok())
}

fn run_sql_script(db_path: &std::path::Path) -> Result<(), String> {
    use std::process::Command;

    let sql_file = File::open("./migrations/migrate_sqlite.sql")
        .map_err(|e| format!("Error opening file: {e}"))?;

    let output = Command::new("sqlite3")
        .arg(db_path)
        .stdin(Stdio::from(sql_file))
        .output()
        .map_err(|e| format!("Error executing command: {e}"))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Errore nell'esecuzione SQL: {error}"));
    }

    Ok(())
}
