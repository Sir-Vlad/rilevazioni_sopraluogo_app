import * as React from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TypeContextType, TypesContext } from "./Context.tsx";
import {
    Climatizzazione,
    Illuminazione,
    MaterialeInfisso,
    NuovoTipo,
    TipoInfisso,
    VetroInfisso
} from "../models/models.tsx";
import { useDatabase } from "@/context/UseProvider.tsx";
import { useErrorContext } from "@/context/ErrorProvider.tsx";

interface TypePayload {
    "materiale_infissi": MaterialeInfisso[],
    "vetro_infissi": VetroInfisso[],
    "climatizzazione": Climatizzazione[],
    "illuminazione": Illuminazione[],
    "tipo_infissi": TipoInfisso[],
}

const TypesProvider = ({ children }: { children: React.ReactNode }) => {
    const {
        needReload,
        registerProvider
    } = useDatabase();
    const providerRef = useRef<{ notifyReloadComplete: () => void; } | null>(null);
    const [ materialiInfissiType, setMaterialiInfissiType ] = useState<string[]>([]);
    const [ vetroInfissiType, setVetroInfissiType ] = useState<string[]>([]);
    const [ climatizzazioneType, setClimatizzazioneType ] = useState<string[]>([]);
    const [ illuminazioneType, setIlluminazioneType ] = useState<string[]>([]);
    const [ tipoInfissi, setTipoInfissi ] = useState<string[]>([]);
    const [ isLoading, setIsLoading ] = useState(true);
    const errorContext = useErrorContext();

    const typeSetters: Record<string, React.Dispatch<React.SetStateAction<string[]>>> = useMemo(() => ({
        climatizzazione: setClimatizzazioneType,
        riscaldamento  : setClimatizzazioneType,
        raffrescamento : setClimatizzazioneType,
        illuminazione  : setIlluminazioneType
    }), []);


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
            setTipoInfissi(data["tipo_infissi"].map(value => value.nome));
        } catch (e) {
            errorContext.addError(e as string);
        } finally {
            setIsLoading(false);
        }
    }, [ errorContext ]);

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

    const addTypeToState = useCallback((tipo: string, name: string) => {
        const setter = typeSetters[tipo.toLowerCase()];
        if (setter) {
            setter((prev) => [ ...prev, name ]);
        }
    }, [ typeSetters ]);

    const insertType = useCallback(async (newType: NuovoTipo) => {
        try {
            const inserted_type: NuovoTipo = await invoke("insert_tipo", { tipo: newType });
            addTypeToState(inserted_type.tipo, inserted_type.name);
        } catch (e) {
            errorContext.addError(e as string);
        }
    }, [ addTypeToState, errorContext ])


    const obj = useMemo(() => {
        return {
            materialiInfissiType,
            vetroInfissiType,
            climatizzazioneType,
            illuminazioneType,
            tipoInfissi,
            isLoading,
            insertType: insertType,
        } as TypeContextType;
    }, [ materialiInfissiType, vetroInfissiType, climatizzazioneType, illuminazioneType, tipoInfissi, isLoading, insertType ]);

    return <TypesContext.Provider value={ obj }>
        { children }
    </TypesContext.Provider>;
};

export default TypesProvider;