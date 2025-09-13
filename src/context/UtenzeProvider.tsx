import {UtenzeContext, UtenzeContextType} from "@/context/Context.tsx";
import {useNotification} from "@/context/NotificationProvider.tsx";
import {useDatabase} from "@/context/UseProvider.tsx";
import {IUtenza} from "@/models/models.tsx";
import {invoke} from "@tauri-apps/api/core";
import * as React from "react";
import {useCallback, useEffect, useMemo, useRef, useState} from "react";

const UtenzeProvider = ({children}: { children: React.ReactNode }) => {
    const {
        needReload,
        registerProvider
    } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [utenze, setUtenze] = useState<IUtenza[]>([]);
    const [loading, setLoading] = useState(true);
    const {addNotification} = useNotification();

    useEffect(() => {
        providerRef.current = registerProvider("utenze");
    }, [registerProvider]);

    const loadUtenze = useCallback(async () => {
        try {
            setLoading(true);
            const utenze: IUtenza[] = await invoke("get_utenze");
            setUtenze(utenze);
            addNotification("Utenze caricate correttamente", "success");
        } catch (e) {
            addNotification(e as string, "error");
        } finally {
            setLoading(false);
        }

    }, [addNotification]);

    const insertUtenza = useCallback(async (utenza: IUtenza) => {
        try {
            setLoading(true);
            const newUtenza: IUtenza = await invoke("insert_utenza", {utenza});
            setUtenze((prev) => {
                return [...prev.filter(value => value.id !== newUtenza.id), newUtenza];
            });
            addNotification("Utenza inserita correttamente", "success");
        } catch (e) {
            addNotification(e as string, "error");
        } finally {
            setLoading(false);
        }
    }, [addNotification]);

    useEffect(() => {
        if (needReload) {
            loadUtenze().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [loadUtenze, needReload]);

    useEffect(() => {
        loadUtenze().catch(console.error);
    }, [loadUtenze]);

    const obj = useMemo(() => (
        {
            data        : utenze,
            isLoading   : loading,
            insertUtenza: insertUtenza
        } as UtenzeContextType), [insertUtenza, loading, utenze]);

    return <UtenzeContext.Provider value={obj}>
        {children}
    </UtenzeContext.Provider>;
};

export default UtenzeProvider;