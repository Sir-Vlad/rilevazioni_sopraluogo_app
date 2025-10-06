import {EdificioContext, EdificioContextType} from "@/context/Context.tsx";
import {useNotification} from "@/context/NotificationProvider.tsx";
import {useDatabase} from "@/context/UseProvider.tsx";
import {IEdificio} from "@/models/models.tsx";
import {invoke} from "@tauri-apps/api/core";
import * as React from "react";
import {useCallback, useEffect, useMemo, useRef, useState} from "react";


const EdificioProvider = ({children}: { children: React.ReactNode }) => {
    const [edifici, setEdifici] = useState<IEdificio[]>([]);
    const {
        needReload,
        registerProvider
    } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const {addNotification} = useNotification();

    useEffect(() => {
        providerRef.current = registerProvider("edificio");
    }, [registerProvider]);

    const loadEdifici = useCallback(async () => {
        try {
            setIsLoading(true);
            const edifici: IEdificio[] = await invoke("get_edifici");
            setEdifici(edifici);
            addNotification("Edifici caricarti correttamente", "success");
        } catch (e) {
            addNotification(e as string, "error");
        } finally {
            setIsLoading(false);
        }
    }, [addNotification]);

    const modifyEdificio = useCallback(async (edificio: IEdificio) => {
        try {
            setIsLoading(true);
            const newEdificio: IEdificio = await invoke("update_edificio", {edificio});
            setEdifici((prev) => {
                const oldEdificio = prev.find(value => value.chiave === edificio.chiave);
                if (oldEdificio) {
                    const mergeObj: IEdificio = {
                        ...oldEdificio,
                        anno_costruzione     : newEdificio.anno_costruzione ?? oldEdificio.anno_costruzione,
                        anno_riqualificazione: newEdificio.anno_riqualificazione ?? oldEdificio.anno_riqualificazione,
                        note_riqualificazione: newEdificio.note_riqualificazione ?? oldEdificio.note_riqualificazione,
                        cappotto             : newEdificio.cappotto ?? oldEdificio.cappotto,
                        isolamento_tetto     : newEdificio.isolamento_tetto ?? oldEdificio.isolamento_tetto
                    };
                    return [...prev.filter(value => value.chiave !== edificio.chiave), mergeObj];
                }
                return prev;
            });
            addNotification("Edificio modificato correttamente", "success");
        } catch (e) {
            addNotification(e as string, "error");
        } finally {
            setIsLoading(false);
        }
    }, [addNotification]);

    // Ricarica i dati quando il database cambia
    useEffect(() => {
        if (needReload) {
            loadEdifici().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [loadEdifici, needReload]);

    // Caricamento iniziale
    useEffect(() => {
        loadEdifici().catch(console.error);
    }, [loadEdifici]);


    const obj: EdificioContextType = useMemo(() => {
        return {
            data          : edifici,
            isLoading     : isLoading,
            modifyEdificio: modifyEdificio,
            setEdifici    : setEdifici,
            error         : null
        };
    }, [edifici, isLoading, modifyEdificio]);

    return <EdificioContext.Provider value={obj}>
        {children}
    </EdificioContext.Provider>;
};

export default EdificioProvider;