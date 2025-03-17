import * as React                                                  from "react";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { useDatabase }                                             from "./UseDatabase.tsx";
import { IStanza }                                                 from "../models/models.tsx";

interface IStanzaContext {
    data: IStanza[];
}

const StanzeContext = createContext<IStanzaContext | null>(null);

const StanzeProvider = ({children}: { children: React.ReactNode }) => {
    const {database}            = useDatabase();
    const [ stanze, setStanze ] = useState<IStanza[]>([]);

    useEffect(() => {
        if (database) {
            const fetchStanze = async () => {
                const res: object[] = await database.select(`
                    SELECT *
                    FROM STANZE;
                `);
                const new_stanze    = res.map((row: Record<string, any>): IStanza => {
                    return Object.fromEntries(Object.entries(row).map(([ key, value ]) => [
                        key.toLowerCase(), value
                    ])) as IStanza;
                });
                setStanze(new_stanze);
            };
            fetchStanze().catch(console.error);
        }
    }, [ database ]);


    const obj: IStanzaContext = useMemo(() => {
        return {
            data: stanze
        };
    }, [ stanze ]);

    return <StanzeContext.Provider value={ obj }>
        { children }
    </StanzeContext.Provider>;
};

export default StanzeProvider;

export const useStanze = () => {
    const stanze = useContext(StanzeContext);
    if (!stanze) {
        throw new Error("useStanze must be used within the DatabaseProvider");
    }
    return stanze;
};