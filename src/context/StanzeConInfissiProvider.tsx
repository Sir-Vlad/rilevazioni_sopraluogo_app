import * as React                                            from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useDatabase }                                       from "./UseProvider.tsx";
import { IStanzaConInfissiContext, StanzeConInfissiContext } from "./Context.tsx";
import { invoke }                                            from "@tauri-apps/api/core";
import { toast }                                             from "sonner";
import { IStanzaConInfissi }                                 from "../models/models.tsx";


const StanzeConInfissiProvider = ({children}: { children: React.ReactNode }) => {
    const [ stanzeConInfissi, setStanzeConInfissi ] = useState<IStanzaConInfissi[]>([]);
    const {
              needReload,
              registerProvider
          } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);


    useEffect(() => {
        providerRef.current = registerProvider("stanze_con_infissi");
    }, [ registerProvider ]);

    const load_stanze_con_infissi = useCallback(async () => {
        setStanzeConInfissi((prev) => prev);
    }, []);

    useEffect(() => {
        if (needReload) {
            load_stanze_con_infissi().then(() => {
                providerRef.current?.notifyReloadComplete();
            });
        }
    }, [ load_stanze_con_infissi, needReload ]);

    const add = useCallback(async (newStanza: IStanzaConInfissi) => {
        try {
            console.log(newStanza);
            await invoke("insert_stanze_con_infissi", {
                newValue: newStanza
            });
            toast.success("Infissi aggiunti");
        } catch (e) {
            toast.error("Errore durante l'aggiunta degli infissi");
            console.error(e);
        }
    }, []);

    const obj = useMemo(() => {
        return {
            data: stanzeConInfissi,
            add : add
        } as IStanzaConInfissiContext;
    }, [ stanzeConInfissi, add ]);

    return <StanzeConInfissiContext.Provider value={ obj }>
        { children }
    </StanzeConInfissiContext.Provider>;
};

export default StanzeConInfissiProvider;