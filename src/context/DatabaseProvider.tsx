import * as React                                              from "react";
import { createContext, useEffect, useMemo, useRef, useState } from "react";
import Database                                                from "@tauri-apps/plugin-sql";
import { invoke }                                              from "@tauri-apps/api/core";

interface DatabaseContextType {
    database: Database | null;
    databasePath: string;
    databaseName: string;
    changeDatabase: (dbName: string) => Promise<void>;
    executeQuery: (query: string, params?: unknown[]) => Promise<void>;
}

export const DatabaseContext = createContext<DatabaseContextType | null>(null);

const DatabaseProvider = ({children}: { children: React.ReactNode }) => {
    const databaseRef                       = useRef<Database | null>(null);
    const [ databaseName, setDatabaseName ] = useState("data");
    const [ databasePath, setDatabasePath ] = useState("");
    const [ database, setDatabase ]         = useState<Database | null>(null);

    const setupDatabase = async (dbName: string) => {
        const dbFilePath = await invoke("get_db_path", {dbName: dbName});
        const dbPath     = `sqlite:${ dbFilePath }`;
        const db         = await Database.load(dbPath);
        await db.execute("PRAGMA foreign_keys=ON");
        await initDatabase(db);
        databaseRef.current = db;
        setDatabase(db);
        setDatabaseName(dbName);
        setDatabasePath(dbPath);
        return "Setup successfully - Database: " + databaseName;
    };

    useEffect(() => {
        const dbName = localStorage.getItem("databaseName");
        setupDatabase(dbName ?? databaseName).catch((error) => {
            console.error(error);
        });
    }, []);

    const changeDatabase = async (dbName: string) => {
        try {
            database?.close();
            console.log("Database closed - Database: " + databaseName);
            await setupDatabase(dbName);
            localStorage.setItem("databaseName", dbName);
        } catch (e) {
            console.error("Errore durante il cambio di database: " + e);
        }
    };

    const execute = async (query: string, params: unknown[] = []) => {
        if (database) {
            try {
                if (databaseRef.current) {
                    await databaseRef.current.execute(query, params);
                } else {
                    console.error("Database non inizializzato");
                }
            } catch (e) {
                console.error("Errore durante l'esecuzione della query: " + e);
            }
        }
    };

    const obj = useMemo(() => {
        return {
            database      : database,
            databaseName  : databaseName,
            databasePath  : databasePath,
            changeDatabase: changeDatabase,
            executeQuery  : execute
        } as DatabaseContextType;
    }, [ database, databasePath, databaseName ]);

    return <DatabaseContext.Provider value={ obj }>
        { children }
    </DatabaseContext.Provider>;
};
export default DatabaseProvider;

const initDatabase = async (db: Database) => {
    await db.execute(`
        CREATE TABLE IF NOT EXISTS INFISSI
        (
            ID        TEXT PRIMARY KEY,
            TIPO      TEXT DEFAULT 'Finestra' CHECK ( TIPO IN ('Finestra', 'Porta')),
            ALTEZZA   REAL NOT NULL,
            LARGHEZZA REAL NOT NULL,
            MATERIALE TEXT NOT NULL,
            VETRO     TEXT NOT NULL
        );
    `);
    await db.execute(`
        CREATE TABLE IF NOT EXISTS STANZE
        (
            ID               INTEGER PRIMARY KEY AUTOINCREMENT,
            FASCICOLO        TEXT NOT NULL,
            PIANO            TEXT NOT NULL,
            ID_SPAZIO        TEXT NOT NULL,
            STANZA           TEXT NOT NULL,
            DESTINAZIONE_USO TEXT NOT NULL,
            ALTEZZA          REAL,
            SPESSORE_MURO    REAL,
            RISCALDAMENTO    TEXT,
            RAFFRESCAMENTO   TEXT,
            ILLUMINAZIONE    TEXT,
            UNIQUE (ID_SPAZIO, STANZA, DESTINAZIONE_USO)
        )
    `);
    await db.execute(`
        CREATE TABLE IF NOT EXISTS STANZE_CON_INFISSI
        (
            ID_INFISSI TEXT NOT NULL,
            ID_STANZA  TEXT NOT NULL,
            FOREIGN KEY (ID_INFISSI) REFERENCES INFISSI (ID),
            FOREIGN KEY (ID_STANZA) REFERENCES STANZE (STANZA),
            PRIMARY KEY (ID_INFISSI, ID_STANZA)
        )
    `);
    await db.execute(`
        CREATE TABLE IF NOT EXISTS TIPO_MATERIALE_INFISSO
        (
            ID        INTEGER PRIMARY KEY AUTOINCREMENT,
            MATERIALE TEXT NOT NULL UNIQUE
        )
    `);
    await db.execute(`
        CREATE TABLE IF NOT EXISTS TIPO_VETRO_INFISSO
        (
            ID    INTEGER PRIMARY KEY AUTOINCREMENT,
            VETRO TEXT NOT NULL UNIQUE
        )
    `);
    await db.execute(`
        CREATE TABLE IF NOT EXISTS TIPO_CLIMATIZZAZIONE
        (
            ID              INTEGER PRIMARY KEY AUTOINCREMENT,
            CLIMATIZZAZIONE TEXT NOT NULL UNIQUE
        )
    `);
    await db.execute(`
        CREATE TABLE IF NOT EXISTS TIPO_ILLUMINAZIONE
        (
            ID        INTEGER PRIMARY KEY AUTOINCREMENT,
            LAMPADINA TEXT NOT NULL UNIQUE
        )
    `);
    try {
        await db.execute(`
            INSERT INTO TIPO_MATERIALE_INFISSO (MATERIALE)
            VALUES ('Legno'),
                   ('Ferro'),
                   ('Alluminio'),
                   ('PVC');
        `);
    } catch { /* empty */
    }
    try {
        await db.execute(`
            INSERT INTO TIPO_VETRO_INFISSO (VETRO)
            VALUES ('Singolo'),
                   ('Doppio'),
                   ('Camera'),
                   ('Plexiglas');
        `);
    } catch { /* empty */
    }
    try {
        await db.execute(`
            INSERT INTO TIPO_CLIMATIZZAZIONE (CLIMATIZZAZIONE)
            VALUES ('Radiatori'),
                   ('Ventilconvettori'),
                   ('Split'),
                   ('A pavimento'),
                   ('Pannelli radianti'),
                   ('Bocchette ad aria')
            ;
        `);
    } catch { /* empty */
    }
    try {
        await db.execute(`
            INSERT INTO TIPO_ILLUMINAZIONE (LAMPADINA)
            VALUES ('Alogene'),
                   ('Neon'),
                   ('Led'),
                   ('Fluorescenza')
            ;
        `);
    } catch { /* empty */
    }
};