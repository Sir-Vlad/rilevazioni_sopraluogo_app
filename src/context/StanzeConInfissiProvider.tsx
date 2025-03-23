import * as React                                                               from "react";
import { useCallback, useMemo, useState }                                       from "react";
import { useDatabase }                                                          from "./UseProvider.tsx";
import { IStanzaConInfissi, IStanzaConInfissiContext, StanzeConInfissiContext } from "./Context.tsx";


const StanzeConInfissiProvider = ({children}: { children: React.ReactNode }) => {
    const [ stanze, setStanze ] = useState<IStanzaConInfissi[]>([]);
    const database              = useDatabase();


    const add = useCallback(async (idStanza: string, idInfisso: string) => {
        console.log("Non implementato");
    }, []);


    const obj = useMemo(() => {
        return {
            data: stanze,
            add : add
        } as IStanzaConInfissiContext;
    }, [ stanze, add ]);

    return <StanzeConInfissiContext.Provider value={ obj }>
        { children }
    </StanzeConInfissiContext.Provider>;
};

export default StanzeConInfissiProvider;