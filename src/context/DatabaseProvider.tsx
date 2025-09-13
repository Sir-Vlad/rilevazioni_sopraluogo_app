import {useNotification} from "@/context/NotificationProvider.tsx";
import {EDIFICIO_CHANGED_EVENT} from "@/context/SelectedEdificioProvider.tsx";
import {invoke} from "@tauri-apps/api/core";
import * as React from "react";
import {useCallback, useEffect, useMemo, useState} from "react";
import {DatabaseContext, DatabaseContextType} from "./Context.tsx";

export const RELOAD_END = "reload-end";

const DatabaseProvider = ({children}: { children: React.ReactNode }) => {
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [needReload, setNeedReload] = useState(false);
    const [pendingProviders, setPendingProviders] = useState(new Set());
    const {addNotification} = useNotification();

    const changeDatabase = useCallback(async (chiave: number) => {
        try {
            setIsLoading(true);
            setError(null);
            await invoke("switch_database", {chiave: chiave});
            addNotification("Cambio file avvenuto con successo", "success");
        } catch (e) {
            setError(e as string);
            addNotification(e as string, "error");
        } finally {
            setIsLoading(false);
        }
    }, [addNotification]);


    useEffect(() => {
        const handleReload = () => {
            console.log("Reload event received");
            setNeedReload(true);
        };

        window.addEventListener(EDIFICIO_CHANGED_EVENT, handleReload);

        return () => {
            window.removeEventListener(EDIFICIO_CHANGED_EVENT, handleReload);
        };
    }, []);

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
            window.dispatchEvent(new CustomEvent(RELOAD_END));
        }
    }, [needReload, pendingProviders]);

    const obj = useMemo(() => {
        return {
            isLoading       : isLoading,
            error           : error,
            needReload      : needReload,
            changeFascicolo : changeDatabase,
            registerProvider: registerProvider
        } as DatabaseContextType;
    }, [error, isLoading, needReload, registerProvider, changeDatabase]);

    return <DatabaseContext.Provider value={obj}>
        {children}
    </DatabaseContext.Provider>;
};
export default DatabaseProvider;