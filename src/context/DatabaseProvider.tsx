import * as React from "react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DatabaseContext, DatabaseContextType } from "./Context.tsx";
import { getFileName } from "../helpers/helpers.ts";
import { useNotification } from "@/context/NotificationProvider.tsx";

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
    const { addNotification } = useNotification();

    const switchDatabase = useCallback(async (dbName: string) => {
        try {
            setIsLoading(true);
            setError(null);
            await invoke("switch_database", { dbName });
            addNotification("Cambio file avvenuto con successo", "success");
        } catch (e) {
            setError(e as string);
            addNotification(e as string, "error")
        } finally {
            setIsLoading(false);
        }
    }, [ addNotification ]);


    useEffect(() => {
        const databaseChangeListener = listen<DatabaseEventPayload>("db-changed", (e) => {
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