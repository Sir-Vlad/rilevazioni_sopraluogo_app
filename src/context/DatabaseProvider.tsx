import * as React                                    from "react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke }                                    from "@tauri-apps/api/core";
import { listen }                                    from "@tauri-apps/api/event";
import { DatabaseContext, DatabaseContextType }      from "./Context.tsx";

interface DatabaseEventPayload {
    type_event: string;
    path: string;
}

const DatabaseProvider = ({children}: { children: React.ReactNode }) => {
    const [ databaseName, setDatabaseName ]         = useState("data");
    const [ databasePath, setDatabasePath ]         = useState("");
    const [ isLoading, setIsLoading ]               = useState(true);
    const [ error, setError ]                       = useState<string | null>(null);
    const [ needReload, setNeedReload ]             = useState(false);
    const [ pendingProviders, setPendingProviders ] = useState(new Set());

    // Inizializzazione iniziale
    useEffect(() => {
        const set_database = async () => {
            const dbName         = localStorage.getItem("databaseName");
            const dbPath: string = await invoke("set_database", {dbName: dbName ?? databaseName});
            setDatabasePath(dbPath);
            setDatabaseName(dbPath.split("/").pop()?.split(".")[0] ?? "");
        };
        set_database().catch(console.error);
    }, []);

    const switchDatabase = useCallback(async (dbName: string) => {
        try {
            setIsLoading(true);
            setError(null);

            await invoke("switch_database", {dbName});
        } catch (e) {
            setError("Errore durante il cambio di database: " + e);
            setIsLoading(false);
        }
    }, []);


    useEffect(() => {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        const databaseChangeListener = listen("database-changed", (e: Event<DatabaseEventPayload>) => {
            console.log("Evento ricevuto: ", e);
            const {payload} = e;

            if (payload.type_event === "database_switched") {
                const databaseName = payload.path.split("/").pop()?.split(".")[0];
                localStorage.setItem("databaseName", databaseName);
                setDatabasePath(payload.path);
                setDatabaseName(databaseName ?? "");
                setNeedReload(true);
                setIsLoading(false);
            }
        });

        return () => {
            databaseChangeListener.then(callback => callback());
        };
    });

    const registerProvider = useCallback((providerId: string) => {
        setPendingProviders(prev => new Set(prev).add(providerId));

        return {
            notifyReloadComplete: () => {
                setPendingProviders(prev => {
                    const newSet = new Set(prev);
                    newSet.delete(providerId);
                    return newSet;
                });
            }
        };
    }, []);

    useEffect(() => {
        if (needReload && pendingProviders.size === 0) {
            setNeedReload(false);
        }
    }, [ needReload, pendingProviders ]);

    const obj = useMemo(() => {
        return {
            databaseName    : databaseName,
            databasePath    : databasePath,
            isLoading       : isLoading,
            error           : error,
            needReload      : needReload,
            changeDatabase  : switchDatabase,
            registerProvider: registerProvider
        } as DatabaseContextType;
    }, [ databaseName, databasePath, error, isLoading, needReload, registerProvider, switchDatabase ]);

    return <DatabaseContext.Provider value={ obj }>
        { children }
    </DatabaseContext.Provider>;
};
export default DatabaseProvider;