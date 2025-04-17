import { EdificioContext, EdificioContextType }              from "@/context/Context.tsx";
import * as React                                            from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { IEdificio }                                         from "@/models/models.tsx";
import { useDatabase }                                       from "@/context/UseProvider.tsx";
import { invoke }                                            from "@tauri-apps/api/core";


const EdificioProvider = ({children}: { children: React.ReactNode }) => {
    const [ edifici, setEdifici ] = useState<IEdificio[]>([]);
    const {
              needReload,
              registerProvider
          } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ error, setError ] = useState<string | null>(null);
    const [ isLoading, setIsLoading ] = useState(true);
    const [ selectedEdificio, setSelectedEdificio ] = useState<string | undefined>(undefined);

    useEffect(() => {
        providerRef.current = registerProvider("edificio");
    }, [ registerProvider ]);

    const loadEdifici = useCallback(async () => {
        try {
            setIsLoading(true);
            setError(null);
            const edifici: IEdificio[] = await invoke("get_edifici");
            setEdifici(edifici);
            setSelectedEdificio([ ...edifici.map(value => value.chiave) ][0]);
        } catch (e) {
            setError("Errore durante il caricamento degli edifici: " + e);
            console.error(e);
        } finally {
            setIsLoading(false);
        }
    }, []);

    // Ricarica i dati quando il database cambia
    useEffect(() => {
        if (needReload) {
            loadEdifici().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [ loadEdifici, needReload ]);

    // Caricamento iniziale
    useEffect(() => {
        loadEdifici().catch(console.error);
    }, [ loadEdifici ]);


    const obj = useMemo(() => {
        return {
            data               : edifici,
            selectedEdificio   : selectedEdificio,
            setSelectedEdificio: setSelectedEdificio,
            error              : error,
            isLoading          : isLoading
        } as EdificioContextType;
    }, [ edifici, error, isLoading, selectedEdificio ]);

    return <EdificioContext.Provider value={ obj }>
        { children }
    </EdificioContext.Provider>;
};

export default EdificioProvider;