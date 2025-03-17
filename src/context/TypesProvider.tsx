import * as React                                                  from "react";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { useDatabase }                                             from "./UseDatabase.tsx";

interface TypeContextType {
    materialiInfissiType: string[];
    vetroInfissiType: string[];
    climatizzazioneType: string[];
    illuminazioneType: string[];
}

const TypesContext = createContext<TypeContextType | null>(null);

const TypesProvider = ({children}: { children: React.ReactNode }) => {
    const {database}                                        = useDatabase();
    const [ materialiInfissiType, setMaterialiInfissiType ] = useState<string[]>([]);
    const [ vetroInfissiType, setVetroInfissiType ]         = useState<string[]>([]);
    const [ climatizzazioneType, setClimatizzazioneType ]   = useState<string[]>([]);
    const [ illuminazioneType, setIlluminazioneType ]       = useState<string[]>([]);


    useEffect(() => {
        if (database) {
            const retrieveMaterialiInfissi = async () => {
                const res: object[] | undefined = await database?.select(`
                    SELECT MATERIALE
                    FROM TIPO_MATERIALE_INFISSO;
                `);
                if (res) {
                    const materiali = res.flatMap(obj => Object.values(obj));
                    setMaterialiInfissiType(materiali);
                }
            };
            const retrieveVetroInfissi     = async () => {
                const res: object[] | undefined = await database?.select(`
                    SELECT VETRO
                    FROM TIPO_VETRO_INFISSO;
                `);
                if (res) {
                    const tipoVetro = res.flatMap(obj => Object.values(obj));
                    setVetroInfissiType(tipoVetro);
                }
            };
            const retrieveClimatizzazione  = async () => {
                const res: object[] | undefined = await database?.select(`
                    SELECT CLIMATIZZAZIONE
                    FROM TIPO_CLIMATIZZAZIONE;
                `);
                if (res) {
                    const climatizzazione = res.flatMap(obj => Object.values(obj));
                    setClimatizzazioneType(climatizzazione);
                }
            };
            const retrieveIlluminazione    = async () => {
                const res: object[] | undefined = await database?.select(`
                    SELECT LAMPADINA
                    FROM TIPO_ILLUMINAZIONE;
                `);
                if (res) {
                    const lampadine = res.flatMap(obj => Object.values(obj));
                    setIlluminazioneType(lampadine);
                }
            };


            retrieveMaterialiInfissi().catch(console.error);
            retrieveVetroInfissi().catch(console.error);
            retrieveClimatizzazione().catch(console.error);
            retrieveIlluminazione().catch(console.error);
        }
    }, [ database ]);


    const obj = useMemo(() => {
        return {
            materialiInfissiType,
            vetroInfissiType,
            climatizzazioneType,
            illuminazioneType
        };
    }, [ materialiInfissiType, vetroInfissiType, climatizzazioneType, illuminazioneType ]);

    return <TypesContext.Provider value={ obj }>
        { children }
    </TypesContext.Provider>;
};

export default TypesProvider;

export const useTypes = () => {
    const type = useContext(TypesContext);
    if (!type) {
        throw new Error("useType must be used within the DatabaseProvider");
    }
    return type;
};