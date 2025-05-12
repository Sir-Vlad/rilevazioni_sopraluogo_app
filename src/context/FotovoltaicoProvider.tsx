import * as React from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { FotovoltaicoContext, FotovoltaicoContextType } from "@/context/Context.tsx";
import { IFotovoltaico } from "@/models/models.tsx";
import { useDatabase } from "@/context/UseProvider.tsx";
import { invoke } from "@tauri-apps/api/core";
import { useErrorContext } from "@/context/ErrorProvider.tsx";

const FotovoltaicoProvider = ({ children }: { children: React.ReactNode }) => {
    const {
        needReload,
        registerProvider
    } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ fotovoltaico, setFotovoltaico ] = useState<IFotovoltaico[]>([]);
    const [ loading, setLoading ] = useState(true);
    const errorContext = useErrorContext();

    useEffect(() => {
        providerRef.current = registerProvider("fotovoltaico");
    }, [ registerProvider ]);

    const loadFotovoltaico = useCallback(async () => {
        try {
            setLoading(true);
            const fotovoltaico: IFotovoltaico[] = await invoke("get_fotovoltaico");
            setFotovoltaico(fotovoltaico);
        } catch (e) {
            errorContext.addError(e as string);
        } finally {
            setLoading(false);
        }

    }, [ errorContext ]);

    useEffect(() => {
        if (needReload) {
            loadFotovoltaico().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [ loadFotovoltaico, needReload ]);

    useEffect(() => {
        loadFotovoltaico().catch(console.error);
    }, [ loadFotovoltaico ]);

    const obj = useMemo(() => ({
        data     : fotovoltaico,
        isLoading: loading
    } as FotovoltaicoContextType), [ loading, fotovoltaico ]);

    return <FotovoltaicoContext.Provider value={ obj }>
        { children }
    </FotovoltaicoContext.Provider>;
};

export default FotovoltaicoProvider;