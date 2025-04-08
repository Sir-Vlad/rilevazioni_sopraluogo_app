import * as React                                            from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { IInfisso }                                          from "../models/models.tsx";
import { useDatabase }                                       from "./UseProvider.tsx";
import { invoke }                                            from "@tauri-apps/api/core";
import { InfissiContext, InfissiContextType }                from "./Context.tsx";

const InfissiProvider = ({children}: { children: React.ReactNode }) => {
    const {
              needReload,
              registerProvider
          } = useDatabase();
    const [ infissi, setInfissi ] = useState<IInfisso[]>([]);
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ error, setError ] = useState<string | null>(null);
    const [ loading, setLoading ] = useState(true);

    useEffect(() => {
        providerRef.current = registerProvider("infissi");
    }, [ registerProvider ]);

    const loadInfissi = useCallback(async () => {
        try {
            setLoading(true);
            setError(null);
            const data: IInfisso[] = await invoke("get_infissi");
            setInfissi(data);
        } catch (e) {
            setError("Errore durante il caricamento degli infissi: " + e);
        } finally {
            setLoading(false);
        }
    }, []);

    // Ricarica i dati quando il database cambia
    useEffect(() => {
        if (needReload) {
            loadInfissi().then(() => {
                providerRef.current?.notifyReloadComplete();
            });
        }
    }, [ loadInfissi, needReload ]);

    // Caricamento iniziale
    useEffect(() => {
        loadInfissi().catch(console.error);
    }, [ loadInfissi ]);

    const insertInfisso = useCallback(async (newInfisso: IInfisso) => {
        try {
            const inserted_infisso: IInfisso = await invoke("insert_infisso", {infisso: newInfisso});
            setInfissi((prev) => [ ...prev, inserted_infisso ]);
        } catch (e) {
            console.error(e);
            throw e;
        }
    }, []);


    const obj = useMemo(() => {
        return {
            data         : infissi,
            insertInfisso: insertInfisso,
            error        : error,
            loading      : loading
        } as InfissiContextType;
    }, [ error, infissi, insertInfisso, loading ]);

    return <InfissiContext.Provider value={ obj }>
        { children }
    </InfissiContext.Provider>;
};

export default InfissiProvider;

