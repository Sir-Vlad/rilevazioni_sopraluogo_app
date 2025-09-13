import {RELOAD_END} from "@/context/DatabaseProvider.tsx";
import {useNotification} from "@/context/NotificationProvider.tsx";
import {useEdifici} from "@/context/UseProvider.tsx";
import {IEdificio} from "@/models/models.tsx";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";
import * as React from "react";
import {createContext, useCallback, useContext, useEffect, useMemo, useState} from "react";

export interface SelectedEdificioContextType {
    edificio: IEdificio | undefined;
    changeEdificio: (edificio: IEdificio) => Promise<void>;
    isLoading: boolean;
    error: string | null;
}

interface EdificioChange {
    type_event: string;
    chiave: string;
}

export const SelectedEdificioContext = createContext<SelectedEdificioContextType | null>(null);

export const EDIFICIO_CHANGED_EVENT = "edificioChanged";

export const SelectedEdificioProvider = ({children}: { children: React.ReactNode }) => {
    const [edificio, setEdificio] = useState<IEdificio>();
    const {addNotification} = useNotification();
    const edificiContext = useEdifici();
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const changeEdificio = useCallback(async (edificio: IEdificio) => {
        try {
            setIsLoading(true);
            setError(null);
            await invoke("set_edificio", {chiave: edificio.chiave});
            setEdificio(edificio);
            addNotification("Cambio edificio avvenuto con successo", "success");
        } catch (e) {
            addNotification(e as string, "error");
        }
    }, [addNotification]);

    useEffect(() => {
        const edificioChangeListener = listen<EdificioChange>("edificio-changed", (e) => {
            console.log("Event change edificio:", e);
            const {payload} = e as { payload: EdificioChange };

            if (payload.type_event === "edificio_change") {
                // launch event for reloading the data
                const edificio = edificiContext.data.find(edificio => {
                    return edificio.chiave === payload.chiave;
                });
                if (edificio) {
                    setEdificio(edificio);
                    window.dispatchEvent(new CustomEvent(EDIFICIO_CHANGED_EVENT));
                } else {
                    setError("Edificio non esiste");
                    addNotification("Edificio non esiste", "error");
                }
            }
        });

        return () => {
            edificioChangeListener
                .then(callback => callback())
                .catch(console.error);
        };
    }, [addNotification, edificiContext.data]);

    useEffect(() => {
        const handleEndReload = () => {
            console.log("Reload end event received");
            setIsLoading(false);
        };

        window.addEventListener(RELOAD_END, handleEndReload);

        return () => {
            window.removeEventListener(RELOAD_END, handleEndReload);
        };
    });


    const obj: SelectedEdificioContextType = useMemo(() => (
        {
            edificio      : edificio,
            changeEdificio: changeEdificio,
            isLoading     : isLoading,
            error         : error
        }), [changeEdificio, edificio, error, isLoading]);


    return <SelectedEdificioContext.Provider value={obj}>
        {children}
    </SelectedEdificioContext.Provider>;
};

export function useSelectedEdificio() {
    const context = useContext(SelectedEdificioContext);
    if (!context) {
        throw new Error("useSelectedEdificio deve essere usato all'interno di SelectedEdificioProvider");
    }
    return context;
}