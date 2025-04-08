import * as React                                            from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useDatabase }                                       from "./UseProvider.tsx";
import { IStanza }                                           from "../models/models.tsx";
import { IStanzaContext, StanzeContext }                     from "./Context.tsx";
import { invoke }                                            from "@tauri-apps/api/core";
import { toast }                                             from "react-toastify";

const StanzeProvider = ({children}: { children: React.ReactNode }) => {
    const {
              needReload,
              registerProvider
          } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ stanze, setStanze ] = useState<IStanza[]>([]);
    const [ error, setError ] = useState<string | null>(null);
    const [ loading, setLoading ] = useState(true);

    useEffect(() => {
        providerRef.current = registerProvider("stanze");
    }, [ registerProvider ]);

    const loadStanze = useCallback(async () => {
        try {
            setLoading(true);
            setError(null);
            const data: IStanza[] = await invoke("get_stanze");
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

    const updateStanza = useCallback(async (newStanza: IStanza) => {
        try {
            await invoke("update_stanza", {stanza: newStanza});
            setStanze((prev) => {
                const newStanzaIndex = prev.findIndex(s => s.id === newStanza.id);
                if (newStanzaIndex !== -1) {
                    prev[newStanzaIndex] = newStanza;
                }
                return prev;
            });
            toast.success("Stanza aggiornata");
        } catch (e) {
            if (e === "Nessun record aggiornato") {
                toast.info("Nessun record aggiornato");
                return;
            }
            console.error("Errore durante l'aggiornamento della stanza: " + e);
            toast.error("Errore durante l'aggiornamento della stanza");
            throw e;
        }

    }, []);


    const obj: IStanzaContext = useMemo(() => {
        return {
            data        : stanze,
            updateStanza: updateStanza,
            error       : error,
            loading     : loading
        };
    }, [ error, loading, stanze, updateStanza ]);

    return <StanzeContext.Provider value={ obj }>
        { children }
    </StanzeContext.Provider>;
};

export default StanzeProvider;