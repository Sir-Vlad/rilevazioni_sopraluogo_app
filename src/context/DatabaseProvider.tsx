import * as React from "react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DatabaseContext, DatabaseContextType } from "./Context.tsx";
import { getFileName } from "../helpers/helpers.ts";

interface DatabaseEventPayload {
    type_event: string;
    path: string;
}

const DatabaseProvider = ({ children }: { children: React.ReactNode }) => {
    const [ databaseName, setDatabaseName ] = useState<string | null>(null);
    const [ databasePath, setDatabasePath ] = useState<string | null>(null);
    const [ isLoading, setIsLoading ] = useState(false);
    const [ error, setError ] = useState<string | null>(null);
    const [ needReload, setNeedReload ] = useState(false);
    const [ pendingProviders, setPendingProviders ] = useState(new Set());

    // Inizializzazione iniziale
    // useEffect(() => {
    //     const set_database = async () => {
    //         try {
    //             setIsLoading(true);
    //             const dbName = localStorage.getItem("databaseName");
    //             if (dbName === null && databaseName === null) {
    //                 setError("Database non settato");
    //                 return;
    //             }
    //
    //             const dbPath: string = await invoke("set_database", {dbName: dbName ?? databaseName});
    //             setDatabasePath(dbPath);
    //             setDatabaseName(getFileName(dbPath) ?? "");
    //             setError(null);
    //         } catch (e) {
    //             setError("Errore durante l'inizializzazione del database");
    //             console.error(e);
    //         } finally {
    //             setIsLoading(false);
    //         }
    //     };
    //     set_database().catch(console.error);
    // }, []);

    const switchDatabase = useCallback(async (dbName: string) => {
        try {
            setIsLoading(true);
            setError(null);
            await invoke("switch_database", { dbName });
        } catch (e) {
            setError(e as string);
        } finally {
            setIsLoading(false);
        }
    }, []);


    useEffect(() => {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        const databaseChangeListener = listen<DatabaseEventPayload>("database-changed", (e: Event<DatabaseEventPayload>) => {
            console.log("Evento ricevuto: ", e);
            const { payload } = e as { payload: DatabaseEventPayload };

            if (payload.type_event === "database_switched") {
                const databaseName = getFileName(payload.path);
                setDatabasePath(payload.path);
                setDatabaseName(databaseName ?? "");
                setNeedReload(true);
                setIsLoading(false);
                setError(null);
            }
        });

        return () => {
            databaseChangeListener
                .then(callback => callback())
                .catch(console.error);
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