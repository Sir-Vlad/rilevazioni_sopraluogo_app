import * as React                                            from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useDatabase }                                       from "./UseProvider.tsx";
import { IStanza }                                           from "../models/models.tsx";
import { IStanzaContext, StanzeContext }                     from "./Context.tsx";
import { invoke }                                            from "@tauri-apps/api/core";

const StanzeProvider = ({children}: { children: React.ReactNode }) => {
    const {
              needReload,
              registerProvider
          }                       = useDatabase();
    const providerRef             = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ stanze, setStanze ]   = useState<IStanza[]>([]);
    const [ error, setError ]     = useState<string | null>(null);
    const [ loading, setLoading ] = useState(true);

    useEffect(() => {
        providerRef.current = registerProvider("stanze");
    }, [ registerProvider ]);

    const loadStanze = useCallback(async () => {
        try {
            setLoading(true);
            setError(null);
            const data: IStanza[] = await invoke("get_stanze");
            console.log("stanze", data);
            setStanze(data);
        } catch (e) {
            setError("Errore durante il caricamento degli infissi: " + e);
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        if (needReload) {
            loadStanze().then(() => {
                providerRef.current?.notifyReloadComplete();
            });
        }
    }, [ loadStanze, needReload ]);

    useEffect(() => {
        loadStanze().catch(console.error);
    }, [ loadStanze ]);

    const updateStanza = (newStanza: IStanza) => {
        // todo: aggiungere al database i nuovi dati
        console.log("updateStanza", newStanza);
    };


    const obj: IStanzaContext = useMemo(() => {
        return {
            data        : stanze,
            updateStanza: updateStanza
        };
    }, [ stanze ]);

    return <StanzeContext.Provider value={ obj }>
        { children }
    </StanzeContext.Provider>;
};

export default StanzeProvider;