import * as React                                    from "react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke }                                    from "@tauri-apps/api/core";
import { TypeContextType, TypesContext }             from "./Context.tsx";

interface TypePayload {
    "materiale_infissi": string[],
    "vetro_infissi": string[],
    "climatizzazione": string[],
    "illuminazione": string[]
}

const TypesProvider = ({children}: { children: React.ReactNode }) => {
    const [ materialiInfissiType, setMaterialiInfissiType ] = useState<string[]>([]);
    const [ vetroInfissiType, setVetroInfissiType ]         = useState<string[]>([]);
    const [ climatizzazioneType, setClimatizzazioneType ]   = useState<string[]>([]);
    const [ illuminazioneType, setIlluminazioneType ]       = useState<string[]>([]);
    const [ error, setError ]                               = useState<string | null>(null);
    const [ isLoading, setIsLoading ]                       = useState(true);


    const loadTypes = useCallback(async () => {
        try {
            setIsLoading(true);
            setError(null);
            const data: TypePayload = await invoke("get_types");
            setMaterialiInfissiType(data["materiale_infissi"]);
            setVetroInfissiType(data["vetro_infissi"]);
            setClimatizzazioneType(data["climatizzazione"]);
            setIlluminazioneType(data["illuminazione"]);
        } catch (e) {
            setError("Errore durante il caricamento degli infissi: " + e);
        } finally {
            setIsLoading(false);
        }
    }, []);

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