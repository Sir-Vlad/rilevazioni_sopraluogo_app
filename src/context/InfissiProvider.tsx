import * as React                                                  from "react";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { IInfisso }                                                from "../models/models.tsx";
import { useDatabase }                                             from "./UseDatabase.tsx";

interface InfissiContextType {
    data: IInfisso[];
    updateInfissi: (newInfisso: IInfisso) => void;
}

const InfissiContext = createContext<InfissiContextType | null>(null);

const InfissiProvider = ({children}: { children: React.ReactNode }) => {
    const {database}              = useDatabase();
    const [ infissi, setInfissi ] = useState<IInfisso[]>([]);

    useEffect(() => {
        if (database) {
            const fetchInfissi = async () => {
                const res: object[] = await database.select(`
                    SELECT *
                    FROM INFISSI
                `);
                const new_infissi   = res.map((row: Record<string, any>): IInfisso => {
                    return Object.fromEntries(Object.entries(row).map(([ key, value ]) => [
                        key.toLowerCase(), value
                    ])) as IInfisso;
                });
                setInfissi(new_infissi);
            };

            fetchInfissi().catch(console.error);
        }
    }, [ database ]);

    const updateInfisso = (newInfisso: IInfisso) => {
        console.log("updateInfisso", newInfisso);

        const columns     = Object.keys(newInfisso).join(", ");
        const placeholder = Object.keys(newInfisso).map((_x, index) => {
            return "$" + (index + 1);
        }).join(", ");
        const query       = `
            INSERT INTO INFISSI (${ columns })
            VALUES (${ placeholder })
        `;
        if (database) {
            database.execute(query, [ ...Object.values(newInfisso) ]).then(r => {
                console.log(r);
            }).catch((e) => {
                console.error("Errore nell'inserimento " + e);
            });
            setInfissi((prev) => [ ...prev, newInfisso ]);
        }
    };


    const obj = useMemo(() => {
        return {
            data         : infissi,
            updateInfissi: updateInfisso
        };
    }, [ infissi ]);

    return <InfissiContext.Provider value={ obj }>
        { children }
    </InfissiContext.Provider>;
};

export default InfissiProvider;

export const useInfissi = () => {
    const infissi = useContext(InfissiContext);
    if (!infissi) {
        throw new Error("useInfissi must be used within an InfissiProvider");
    }
    return infissi;
};