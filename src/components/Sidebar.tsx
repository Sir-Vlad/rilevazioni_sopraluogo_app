import { useEffect, useState }                                              from "react";
import { faArrowLeft, faArrowRight, faArrowUpRightFromSquare, faFileLines } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon }                                                  from "@fortawesome/react-fontawesome";
import { invoke }                                                           from "@tauri-apps/api/core";
import { open }                                                             from "@tauri-apps/plugin-dialog";

const Sidebar = () => {
    const [ isOpen, setIsOpen ] = useState(false);

    const toggleSidebar                         = () => {
        setIsOpen(!isOpen);
    };
    const [ databasesFiles, setDatabasesFiles ] = useState<string[]>([]);

    useEffect(() => {
        const retrieveNameDatabases = async () => {
            const dbs: string[] = await invoke("get_all_name_database");
            setDatabasesFiles(dbs);
        };
        retrieveNameDatabases().then();
    }, []);


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
                    { databasesFiles.map((database, i) => {
                        return <div className="grid grid-cols-8 items-center h-full" key={ i }>
                            <button
                                className="col-span-5 flex items-center rounded hover:bg-gray-700 cursor-pointer"
                                onClick={ () => {
                                } }>
                                <FontAwesomeIcon icon={ faFileLines }
                                                 className="flex items-center p-2" />
                                <span className="ml-3">{ database.split(".")[0] }</span>
                            </button>
                            <button className="col-span-2 rounded hover:bg-gray-700 cursor-pointer p-2"
                                    onClick={ () => {
                                    } }>
                                <FontAwesomeIcon icon={ faArrowUpRightFromSquare } />
                            </button>
                        </div>;
                    }) }
                    {/* Btn add nuovo fascicolo */ }
                    <button className="mt-3 p-1 rounded bg-gray-700 hover:bg-gray-600"
                            onClick={ async () => {
                                const file = await open({
                                    title    : "Seleziona il file da caricare",
                                    multiple : false,
                                    directory: false,
                                    filters  : [
                                        {
                                            name      : "Excel file",
                                            extensions: [ "xlsx", "xls" ]
                                        },
                                        {
                                            name      : "CSV file",
                                            extensions: [ "csv" ]
                                        }
                                    ]
                                });
                                console.log(file);
                                /* Passare il path a rust che ne elabora il contenuto (con polars) e lo aggiunge al db */
                                await invoke("elaborate_file", {path: file});
                            } }
                    >Aggiungi
                    </button>
                </div>
            </ul>
        </nav> }
    </aside>;
};

export default Sidebar;