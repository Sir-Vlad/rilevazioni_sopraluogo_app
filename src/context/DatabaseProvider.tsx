import * as React                                                  from "react";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import Database                                                    from "@tauri-apps/plugin-sql";
import { invoke }                                                  from "@tauri-apps/api/core";

interface DatabaseContextType {
    database: Database | null;
    changeDatabase: (dbName: string) => void;
}

const DatabaseContext = createContext<DatabaseContextType | null>(null);

const DatabaseProvider = ({children}: { children: React.ReactNode }) => {
    const [ databaseName, setDatabaseName ] = useState("data");
    const [ database, setDatabase ]         = useState<Database | null>(null);

    useEffect(() => {
        const setupDatabase = async () => {
            const dbPath = await invoke("get_db_path", {dbName: databaseName});
            const db     = await Database.load(`sqlite:${ dbPath }`);
            await db.execute("PRAGMA foreign_keys=ON");
            await initDatabase(db);
            setDatabase(db);
        };

        setupDatabase().catch((error) => {
            console.error(error);
            alert(error);
        });
    }, [ databaseName ]);

    const changeDatabase = (dbName: string) => {
        setDatabaseName(dbName);
    };

    const obj = useMemo(() => {
        return {
            database,
            changeDatabase
        };
    }, [ database ]);

    return <DatabaseContext.Provider value={ obj }>
        { children }
    </DatabaseContext.Provider>;
};
export default DatabaseProvider;

export const useDatabase = () => {
    const context = useContext(DatabaseContext);
    if (!context) {
        throw new Error("useDatabase must be used within the DatabaseProvider");
    }
    return context;
};

const initDatabase = async (db: Database) => {
    await db.execute(`
        CREATE TABLE IF NOT EXISTS INFISSI
        (
            ID        TEXT PRIMARY KEY,
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
            SDV              TEXT NOT NULL,
            PIANO            TEXT NOT NULL,
            ID_SPAZIO        TEXT NOT NULL,
            STANZA           TEXT NOT NULL,
            DESTINAZIONE_USO TEXT NOT NULL,
            ALTEZZA          REAL,
            SPESSORE_MURO    REAL,
            RISCALDAMENTO    TEXT,
            RAFFRESCAMENTO   TEXT,
            ILLUMINAZIONE    TEXT,
            UNIQUE (SDV, ID_SPAZIO, STANZA, DESTINAZIONE_USO)
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