import { useCallback, useEffect, useState } from "react";
import {
    faArrowLeft,
    faArrowRight,
    faArrowUpRightFromSquare,
    faCheck,
    faFileLines
}                                           from "@fortawesome/free-solid-svg-icons";
import {
    FontAwesomeIcon
}                                           from "@fortawesome/react-fontawesome";
import { invoke }                           from "@tauri-apps/api/core";
import { open }                             from "@tauri-apps/plugin-dialog";

import { useDatabase } from "../context/UseProvider.tsx";
import { toast }       from "react-toastify";
import { getFileName } from "../helpers/helpers.tsx";

const Sidebar = () => {
    const [ isOpen, setIsOpen ] = useState(false);
    const [ databasesFiles, setDatabasesFiles ] = useState<string[]>([]);
    const database = useDatabase();
    const [ selectedDatabase, setSelectedDatabase ] = useState<string>(database.databaseName);

    const toggleSidebar = () => {
        setIsOpen(!isOpen);
    };

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
            const path_db: string = await invoke("insert_stanze", {
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

    return <aside className={ `bg-gray-800 text-white transition-all ${ isOpen ? "w-64" : "w-16" } p-4` }>
        <div className="flex justify-between items-center mb-6">
            { isOpen && <h2 className="text-xl font-bold">Fascicoli</h2> }
            <button
                onClick={ toggleSidebar }
                className="text-white p-2 rounded hover:bg-gray-700"
            >
                { isOpen ? <FontAwesomeIcon icon={ faArrowLeft } /> : <FontAwesomeIcon icon={ faArrowRight } /> }
            </button>
        </div>
        { isOpen && <nav>
            <ul className="space-y-2">
                <div className="h-full flex flex-col">
                    {/* Btn dei vari fascicoli */ }
                    { databasesFiles.map((databaseFile, i) => {
                        const nameDatabase = getFileName(databaseFile);
                        if (!nameDatabase) return;
                        return <div className="grid grid-cols-10 items-center h-full" key={ i }>
                            <button
                                className="col-span-7 flex items-center rounded hover:bg-gray-700 cursor-pointer"
                                onClick={ async () => {
                                    await database.changeDatabase(nameDatabase);
                                    setSelectedDatabase(nameDatabase);
                                } }>
                                <FontAwesomeIcon icon={ faFileLines }
                                                 className="flex items-center p-2" />
                                <span className="ml-2 truncate">{ nameDatabase }</span>
                            </button>
                            <button className="col-span-2 rounded hover:bg-gray-700 cursor-pointer p-2"
                                    onClick={ () => {
                                    } }>
                                <FontAwesomeIcon icon={ faArrowUpRightFromSquare } />
                            </button>
                            <div
                                className={ `col-span-1 flex items-center justify-center ${ selectedDatabase === nameDatabase ? "" : "hidden" }` }>
                                <FontAwesomeIcon icon={ faCheck } />
                            </div>
                        </div>;
                    }) }
                    {/* Btn add nuovo fascicolo */ }
                    <button className="mt-3 p-1 rounded bg-gray-700 hover:bg-gray-600"
                            onClick={ addNewFascicolo }
                    >Aggiungi
                    </button>
                </div>
            </ul>
        </nav> }
    </aside>;
};

export default Sidebar;