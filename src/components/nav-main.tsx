"use client";

import {Collapsible, CollapsibleContent, CollapsibleTrigger} from "@/components/ui/collapsible.tsx";
import {
    SidebarGroup,
    SidebarGroupAction,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarMenuSub
} from "@/components/ui/sidebar";
import {useNotification} from "@/context/NotificationProvider.tsx";
import {useSelectedEdificio} from "@/context/SelectedEdificioProvider.tsx";
import {useEdifici} from "@/context/UseProvider.tsx";
import {IEdificio} from "@/models/models.tsx";
import {invoke} from "@tauri-apps/api/core";
import {open} from "@tauri-apps/plugin-dialog";
import {Building, Check, ChevronRight, FileText, Plus} from "lucide-react";
import {useEffect, useMemo, useState} from "react";

export function NavMain({valueSearch}: Readonly<{ valueSearch: string | null }>) {
    const selectedEdificio = useSelectedEdificio();
    const edificiContext = useEdifici();
    const {addNotification} = useNotification();

    const [fascicoli, setFascicoli] = useState(new Map<number, IEdificio[]>());
    const [filteredFascicolo, setFilteredFascicolo] = useState(new Map<number, IEdificio[]>);
    const fascicoliRender = useMemo(() => {
        if (valueSearch !== null) {
            return Array.from(filteredFascicolo);
        }
        return Array.from(fascicoli);
    }, [valueSearch, fascicoli, filteredFascicolo]);
    const [selectedFascicolo, setSelectedFascicolo] = useState<number>();

    useEffect(() => {
        const newFascicoli = new Map<number, IEdificio[]>();

        edificiContext.data
                      .toSorted((a, b) => a.fascicolo - b.fascicolo)
                      .forEach(value => {
                          const chiave = value.fascicolo;

                          if (!newFascicoli.has(chiave)) {
                              newFascicoli.set(chiave, []);
                          }
                          newFascicoli.get(chiave)!.push(value);
                      });

        setFascicoli(newFascicoli);
    }, [edificiContext.data]);


    useEffect(() => {
        if (valueSearch) {
            const fascicoli_filtered = new Map([...fascicoli].filter(([fascicolo]) => {
                return fascicolo.toString().startsWith(valueSearch);
            }));

            setFilteredFascicolo(fascicoli_filtered);

        } else {
            setFilteredFascicolo(new Map<number, IEdificio[]>);
        }
    }, [fascicoli, valueSearch]);

    const addNewFascicolo = async () => {
        const path_file = await open({
            title    : "Seleziona il file da caricare",
            multiple : false,
            directory: false,
            filters  : [
                {
                    name      : "Excel file",
                    extensions: ["xlsx", "xls"]
                }
            ]
        });
        if (!path_file) {
            return;
        }
        console.log(path_file);


        /* Passare il path a rust che ne elabora il contenuto (con polars) e imposta il database */
        try {
            await invoke("add_new_fascicolo_from_xlsx", {
                path: path_file
            });
            addNotification("Inserimento avvenuto con successo", "success");
        } catch (e) {
            addNotification(e as string, "error");
        }
    };

    const renderCheckedEdificio = (edificio: IEdificio) => {
        const visibilityClass = selectedEdificio.edificio?.chiave === edificio.chiave ? "" : "hidden";

        return (
            <div className={`flex justify-end ${visibilityClass}`}>
                <Check/>
            </div>);
    };

    const renderLoadingEdificio = () => {
        return (
            <div className="flex justify-end">
                <div className="animate-spin rounded-full h-4 w-4 border-t-2 border-b-2 border-white"></div>
            </div>);
    };

    return (
        <SidebarGroup>
        <SidebarGroupLabel>Fascicoli</SidebarGroupLabel>
        <SidebarGroupAction title="Aggiungi Fascicolo" onClick={() => void addNewFascicolo()}>
            <Plus/> <span className="sr-only">Aggiungi Fascicolo</span>
        </SidebarGroupAction>
        <SidebarMenu>
            {fascicoliRender.map(([fascicolo, edifici]) => {
                return <Collapsible defaultOpen={false} className="group/collapsible" key={fascicolo}>
                        <SidebarMenuItem>
                            <CollapsibleTrigger asChild>
                                <SidebarMenuButton tooltip={fascicolo.toString()}>
                                    <FileText/>
                                    <div className="flex flex-row justify-between content-center w-full">
                                        <p>{fascicolo}</p>
                                        {selectedFascicolo === fascicolo && <Check/>}
                                    </div>
                                    <ChevronRight
                                        className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90"/>
                                </SidebarMenuButton>
                            </CollapsibleTrigger>
                            <CollapsibleContent>
                                <SidebarMenuSub>
                                    {edifici.map(edificio => {
                                        return <SidebarMenuItem key={edificio.chiave}>
                                                    <SidebarMenuButton asChild tooltip={fascicolo.toString()}
                                                                       onClick={async () => {
                                                                           setSelectedFascicolo(fascicolo);
                                                                           await selectedEdificio.changeEdificio(edificio);
                                                                       }}>
                                                        <div className="flex items-center justify-between">
                                                            <div className="flex items-center gap-2">
                                                                <Building size="16"/>
                                                                <span>{edificio.chiave}</span>
                                                            </div>
                                                            {selectedEdificio.isLoading ? renderLoadingEdificio() : renderCheckedEdificio(edificio)}
                                                        </div>
                                                    </SidebarMenuButton>
                                            </SidebarMenuItem>;
                                    })}
                                </SidebarMenuSub>
                            </CollapsibleContent>
                        </SidebarMenuItem>
                    </Collapsible>;
            })}
        </SidebarMenu>
    </SidebarGroup>);
}
