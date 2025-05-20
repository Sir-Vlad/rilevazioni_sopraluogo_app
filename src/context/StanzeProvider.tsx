import * as React from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useDatabase, useEdifici } from "./UseProvider.tsx";
import { IStanza } from "../models/models.tsx";
import { IStanzaContext, StanzeContext } from "./Context.tsx";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import { useErrorContext } from "@/context/ErrorProvider.tsx";

const StanzeProvider = ({ children }: { children: React.ReactNode }) => {
    const {
        needReload,
        registerProvider
    } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ stanze, setStanze ] = useState<IStanza[]>([]);
    const [ loading, setLoading ] = useState(true);
    const errorContext = useErrorContext();
    const { selectedEdificio } = useEdifici();

    useEffect(() => {
        providerRef.current = registerProvider("stanze");
    }, [ registerProvider ]);

    const loadStanze = useCallback(async () => {
        try {
            setLoading(true);
            const data: IStanza[] = await invoke("get_stanze");
            setStanze(data);
        } catch (e) {
            errorContext.addError(e as string);
        } finally {
            setLoading(false);
        }
    }, [ errorContext ]);

    useEffect(() => {
        if (needReload) {
            loadStanze().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [ loadStanze, needReload ]);

    useEffect(() => {
        loadStanze().catch(console.error);
    }, [ loadStanze ]);

    const updateStanza = useCallback(async (newStanza: IStanza) => {
        const updateStanzaProperties = (oldStanza: IStanza, newStanza: IStanza): IStanza => ({
            ...oldStanza,
            altezza       : newStanza.altezza ?? oldStanza.altezza,
            spessore_muro : newStanza.spessore_muro ?? oldStanza.spessore_muro,
            riscaldamento : newStanza.riscaldamento ?? oldStanza.riscaldamento,
            raffrescamento: newStanza.raffrescamento ?? oldStanza.raffrescamento,
            illuminazione : newStanza.illuminazione ?? oldStanza.illuminazione,
            infissi       : [ ...(oldStanza.infissi ?? []), ...(newStanza.infissi ?? []) ],
        });


        try {
            await invoke("update_stanza", { stanza: newStanza });
            setStanze((prevStanze) => prevStanze.map((stanza) => stanza.id === newStanza.id ? updateStanzaProperties(stanza, newStanza) : stanza));
            toast.success(`Stanza ${ newStanza.stanza } aggiornata`);
        } catch (e) {
            if (e === "Nessun record aggiornato") {
                toast.info(e as string);
                return;
            }
            errorContext.addError(e as string);
        }

    }, [ errorContext ]);


    const obj: IStanzaContext = useMemo(() => {
        return {
            data        : stanze.filter(value => value.chiave === selectedEdificio) ?? [],
            updateStanza: updateStanza,
            loading     : loading
        };
    }, [loading, selectedEdificio, stanze, updateStanza]);

    return <StanzeContext.Provider value={ obj }>
        { children }
    </StanzeContext.Provider>;
};

export default StanzeProvider;