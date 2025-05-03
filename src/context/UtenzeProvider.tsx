import * as React                                            from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { UtenzeContext, UtenzeContextType }                  from "@/context/Context.tsx";
import { IUtenza }                                           from "@/models/models.tsx";
import { useDatabase }                                       from "@/context/UseProvider.tsx";
import { invoke }                                            from "@tauri-apps/api/core";
import { useErrorContext }                                   from "@/context/ErrorProvider.tsx";

const UtenzeProvider = ({children}: { children: React.ReactNode }) => {
    const {
              needReload,
              registerProvider
          } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ utenze, setUtenze ] = useState<IUtenza[]>([]);
    const [ loading, setLoading ] = useState(true);
    const errorContext = useErrorContext();

    useEffect(() => {
        providerRef.current = registerProvider("utenze");
    }, [ registerProvider ]);

    const loadUtenze = useCallback(async () => {
        try {
            setLoading(true);
            const utenze: IUtenza[] = await invoke("get_utenze");
            setUtenze(utenze);
        } catch (e) {
            if (typeof e === "string") {
                errorContext.addError(e);
                console.error(e);
            }
        } finally {
            setLoading(false);
        }

    }, []);

    useEffect(() => {
        if (needReload) {
            loadUtenze().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [ loadUtenze, needReload ]);

    useEffect(() => {
        loadUtenze().catch(console.error);
    }, [ loadUtenze ]);

    const obj = useMemo(() => ({
        data: utenze,
        isLoading: loading,
    } as UtenzeContextType), [loading, utenze]);

    return <UtenzeContext.Provider value={ obj }>
        { children }
    </UtenzeContext.Provider>;
};

export default UtenzeProvider;