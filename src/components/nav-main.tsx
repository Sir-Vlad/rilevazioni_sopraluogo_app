"use client";

import {
  SidebarGroup,
  SidebarGroupAction,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem
}                                                       from "@/components/ui/sidebar";
import { useCallback, useEffect, useState }             from "react";
import { useDatabase }                                  from "@/context/UseProvider.tsx";
import { invoke }                                       from "@tauri-apps/api/core";
import { open }                                         from "@tauri-apps/plugin-dialog";
import { toast }                                        from "react-toastify";
import { Check, FileSpreadsheet, MoreHorizontal, Plus } from "lucide-react";
import { getFileName }                                  from "@/helpers/helpers.tsx";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
}                                                       from "@/components/ui/dropdown-menu";

export function NavMain() {
    const [ databasesFiles, setDatabasesFiles ] = useState<string[]>([]);
    const database = useDatabase();
    const [ selectedDatabase, setSelectedDatabase ] = useState<string>(database.databaseName);

    const retrieveNameDatabases = useCallback(async () => {
        const dbs: string[] = await invoke("get_all_name_database");
        setDatabasesFiles(dbs);
    }, []);

    useEffect(() => {
        retrieveNameDatabases().catch(console.error);
    }, [ retrieveNameDatabases ]);

    useEffect(() => {
        setSelectedDatabase(database.databaseName);
    }, [ database.databaseName ]);

    const addNewFascicolo = async () => {
        const file = await open({
            title    : "Seleziona il file da caricare",
            multiple : false,
            directory: false,
            filters  : [
                {
                    name      : "Excel file",
                    extensions: [ "xlsx", "xls" ]
                }
            ]
        });
        /* Passare il path a rust che ne elabora il contenuto (con polars) e mi ritorna un json del contenuto del file*/
        try {
            const path_db: string = await invoke("init_to_excel", {
                path: file
            });
            setDatabasesFiles((prev) => [ ...prev, path_db ]);
            console.log("Inserimento avvenuto con successo");
            toast.success("Inserimento avvenuto con successo");
        } catch (e) {
            console.log("Errore durante l'inserimento: " + e);
            toast.error("Errore durante il cambio di database");
        }
    };

    return (
        <SidebarGroup>
            <SidebarGroupLabel>Fascicoli</SidebarGroupLabel>
            <SidebarGroupAction title="Aggiungi Fascicolo" onClick={ addNewFascicolo }>
                <Plus /> <span className="sr-only">Aggiungi Fascicolo</span>
            </SidebarGroupAction>
            <SidebarMenu>
                { databasesFiles.map((file) => {
                    const nameDatabase = getFileName(file);
                    return <SidebarMenuItem key={ file }>
                        <div className="flex grow-1">
                            <SidebarMenuButton asChild tooltip={ file } onClick={ async () => {
                                await database.changeDatabase(nameDatabase);
                                setSelectedDatabase(nameDatabase);
                            } }>
                                <div className="flex items-center">
                                    <FileSpreadsheet />
                                    <span>{ file }</span>
                                    <div
                                        className={ `flex w-full justify-end ${ selectedDatabase === nameDatabase ? "" : "hidden" }` }>
                                        <Check />
                                    </div>
                                </div>
                            </SidebarMenuButton>
                        </div>
                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <SidebarMenuAction>
                                    <MoreHorizontal />
                                </SidebarMenuAction>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent side="right" align="start">
                                <DropdownMenuItem>
                                    <span>Esporta in excel</span>
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </SidebarMenuItem>;
                }) }
            </SidebarMenu>
        </SidebarGroup>
    );
}
