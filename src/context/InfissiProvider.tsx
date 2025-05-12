import * as React from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { IInfisso } from "../models/models.tsx";
import { useDatabase } from "./UseProvider.tsx";
import { invoke } from "@tauri-apps/api/core";
import { InfissiContext, InfissiContextType } from "./Context.tsx";
import { useErrorContext } from "./ErrorProvider.tsx";

const InfissiProvider = ({ children }: { children: React.ReactNode }) => {
    const {
        needReload,
        registerProvider
    } = useDatabase();
    const [ infissi, setInfissi ] = useState<IInfisso[]>([]);
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ loading, setLoading ] = useState(true);
    const errorContext = useErrorContext();

    useEffect(() => {
        providerRef.current = registerProvider("infissi");
    }, [ registerProvider ]);

    const loadInfissi = useCallback(async () => {
        try {
            setLoading(true);
            const data: IInfisso[] = await invoke("get_infissi");
            setInfissi(data);
        } catch (e) {
            errorContext.addError(e as string);
        } finally {
            setLoading(false);
        }
    }, [ errorContext ]);

    // Ricarica i dati quando il database cambia
    useEffect(() => {
        if (needReload) {
            loadInfissi().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [ loadInfissi, needReload ]);

    // Caricamento iniziale
    useEffect(() => {
        loadInfissi().catch(console.error);
    }, [ loadInfissi ]);

    const insertInfisso = useCallback(async (newInfisso: IInfisso) => {
        try {
            const inserted_infisso: IInfisso = await invoke("insert_infisso", { infisso: newInfisso });
            setInfissi((prev) => [ ...prev, inserted_infisso ]);
        } catch (e) {
            errorContext.addError(e as string);
        }
    }, [ errorContext ]);

    const modifyInfisso = useCallback(async (infisso: IInfisso) => {
        try {
            const inserted_infisso: IInfisso = await invoke("update_infisso", { infisso: infisso });
            setInfissi((prev) => [ ...prev.filter(i => i.id !== infisso.id), inserted_infisso ]);
        } catch (e) {
            errorContext.addError(e as string);
        }
    }, [ errorContext ]);


    const obj = useMemo(() => {
        return {
            data         : infissi,
            insertInfisso: insertInfisso,
            modifyInfisso: modifyInfisso,
            isLoading    : loading
        } as InfissiContextType;
    }, [ infissi, insertInfisso, loading, modifyInfisso ]);

    return <InfissiContext.Provider value={ obj }>
        { children }
    </InfissiContext.Provider>;
};

export default InfissiProvider;

