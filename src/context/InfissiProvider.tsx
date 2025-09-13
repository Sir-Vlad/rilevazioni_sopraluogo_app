import {useSelectedEdificio} from "@/context/SelectedEdificioProvider.tsx";
import {invoke} from "@tauri-apps/api/core";
import * as React from "react";
import {useCallback, useEffect, useMemo, useRef, useState} from "react";
import {IInfisso} from "../models/models.tsx";
import {InfissiContext, InfissiContextType} from "./Context.tsx";
import {useNotification} from "./NotificationProvider.tsx";
import {useDatabase} from "./UseProvider.tsx";

const InfissiProvider = ({children}: { children: React.ReactNode }) => {
    const {
        needReload,
        registerProvider
    } = useDatabase();
    const [infissi, setInfissi] = useState<IInfisso[]>([]);
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [loading, setLoading] = useState(true);
    const {addNotification} = useNotification();
    const {edificio} = useSelectedEdificio();

    useEffect(() => {
        providerRef.current = registerProvider("infissi");
    }, [registerProvider]);

    const loadInfissi = useCallback(async () => {
        try {
            setLoading(true);
            const data: IInfisso[] = await invoke("get_infissi");
            setInfissi(data);
            addNotification("Infissi caricati correttamente", "success");
        } catch (e) {
            addNotification(e as string, "error");
        } finally {
            setLoading(false);
        }
    }, [addNotification]);

    // Ricarica i dati quando il database cambia
    useEffect(() => {
        if (needReload) {
            loadInfissi().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [loadInfissi, needReload]);

    // Caricamento iniziale
    useEffect(() => {
        loadInfissi().catch(console.error);
    }, [loadInfissi]);

    const insertInfisso = useCallback(async (newInfisso: IInfisso) => {
        try {
            const insertedInfisso: IInfisso = await invoke("insert_infisso", {infisso: newInfisso});
            setInfissi((prev) => [...prev, insertedInfisso]);
            addNotification(`Infisso ${insertedInfisso.id} inserito correttamente`, "success");
        } catch (e) {
            addNotification(e as string, "error");
        }
    }, [addNotification]);

    const modifyInfisso = useCallback(async (infisso: IInfisso) => {
        try {
            const updatedInfisso: IInfisso = await invoke("update_infisso", {infisso: infisso});
            setInfissi((prev) => [...prev.filter(i => i.id !== infisso.id), updatedInfisso]);
            addNotification(`Infisso ${updatedInfisso.id} modificato correttamente`, "success");
        } catch (e) {
            addNotification(e as string, "error");
        }
    }, [addNotification]);


    const obj = useMemo(() => {
        return {
            data         : edificio === undefined ? [] : infissi.filter(value => value.id_edificio ===
                                                                                 edificio.chiave),
            insertInfisso: insertInfisso,
            modifyInfisso: modifyInfisso,
            isLoading    : loading
        } as InfissiContextType;
    }, [edificio, infissi, insertInfisso, loading, modifyInfisso]);

    return <InfissiContext.Provider value={obj}>
        {children}
    </InfissiContext.Provider>;
};

export default InfissiProvider;

