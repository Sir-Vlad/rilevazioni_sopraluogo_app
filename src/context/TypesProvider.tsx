import * as React                                                         from "react";
import { useCallback, useEffect, useMemo, useRef, useState }              from "react";
import { invoke }                                                         from "@tauri-apps/api/core";
import { TypeContextType, TypesContext }                                  from "./Context.tsx";
import { Climatizzazione, Illuminazione, MaterialeInfisso, VetroInfisso } from "../models/models.tsx";
import { useDatabase }                                                    from "@/context/UseProvider.tsx";
import { useErrorContext }                                                from "@/context/ErrorProvider.tsx";

interface TypePayload {
    "materiale_infissi": MaterialeInfisso[],
    "vetro_infissi": VetroInfisso[],
    "climatizzazione": Climatizzazione[],
    "illuminazione": Illuminazione[]
}

const TypesProvider = ({children}: { children: React.ReactNode }) => {
    const {
              needReload,
              registerProvider
          } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ materialiInfissiType, setMaterialiInfissiType ] = useState<string[]>([]);
    const [ vetroInfissiType, setVetroInfissiType ] = useState<string[]>([]);
    const [ climatizzazioneType, setClimatizzazioneType ] = useState<string[]>([]);
    const [ illuminazioneType, setIlluminazioneType ] = useState<string[]>([]);
    const [ isLoading, setIsLoading ] = useState(true);
    const errorContext = useErrorContext();

    useEffect(() => {
        providerRef.current = registerProvider("tipi");
    }, [ registerProvider ]);

    const loadTypes = useCallback(async () => {
        try {
            setIsLoading(true);
            const data: TypePayload = await invoke("get_all_tipi");
            setMaterialiInfissiType(data["materiale_infissi"].map(value => value.materiale));
            setVetroInfissiType(data["vetro_infissi"].map(value => value.vetro));
            setClimatizzazioneType(data["climatizzazione"].map(value => value.climatizzazione));
            setIlluminazioneType(data["illuminazione"].map(value => value.lampadina));
        } catch (e) {
            if (typeof e === "string") {
                errorContext.addError(e);
                console.error(e);
            }
        } finally {
            setIsLoading(false);
        }
    }, [errorContext]);

    useEffect(() => {
        if (needReload) {
            loadTypes().then(() => {
                providerRef.current?.notifyReloadComplete();
            }).catch(console.error);
        }
    }, [ loadTypes, needReload ]);

    useEffect(() => {
        loadTypes().catch(console.error);
    }, [ loadTypes ]);

    const obj = useMemo(() => {
        return {
            materialiInfissiType,
            vetroInfissiType,
            climatizzazioneType,
            illuminazioneType,
            isLoading
        } as TypeContextType;
    }, [ materialiInfissiType, vetroInfissiType, climatizzazioneType, illuminazioneType, isLoading ]);

    return <TypesContext.Provider value={ obj }>
        { children }
    </TypesContext.Provider>;
};

export default TypesProvider;