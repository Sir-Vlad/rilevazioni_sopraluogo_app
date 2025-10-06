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

interface EventBackend {
    type_event: string;
}

interface EdificioChange extends EventBackend {
    chiave: string;
}

interface NewEdificio extends EventBackend {
    edifici: IEdificio[];
    edificio_selected: string;
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

    function setEdificioSelected(edificio: IEdificio) {
        setEdificio(edificio);
        window.dispatchEvent(new CustomEvent(EDIFICIO_CHANGED_EVENT));
    }


    useEffect(() => {
        const edificioChangeListener = listen<EdificioChange>("edificio", (e) => {
            console.log("Event change edificio arrived:", e);
            const {payload} = e as { payload: EventBackend };

            if (payload.type_event === "ChangedEdificio") {
                console.log("Event change edificio:", payload);
                const event = payload as EdificioChange;

                // launch event for reloading the data
                const edificio = edificiContext.data.find(edificio => {
                    return edificio.chiave === event.chiave;
                });
                if (edificio) {
                    setEdificioSelected(edificio);
                } else {
                    setError("Edificio non esiste");
                    addNotification("Edificio non esiste", "error");
                }
            } else if (payload.type_event === "NewEdificio") {
                console.log("Event new edificio:", payload);
                const event = payload as NewEdificio;

                edificiContext.setEdifici((prevState) => {
                    return [...prevState, ...event.edifici];
                });
                const edificio = event.edifici.find(edificio => edificio.chiave === event.edificio_selected)!;
                setEdificioSelected(edificio);
            } else {
                setIsLoading(false);
                setError("Edificio non esiste");
                addNotification("Edificio non esiste", "error");
            }
        });

        return () => {
            edificioChangeListener
                .then(callback => callback())
                .catch(console.error);
        };
    }, [addNotification, edificiContext, edificiContext.data]);

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