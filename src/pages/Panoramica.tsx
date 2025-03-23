import { useEffect, useMemo, useRef, useState } from "react";
import { AgGridReact }                          from "ag-grid-react";
import { ColDef }                               from "ag-grid-community";
import { IStanza }                              from "../models/models.tsx";
import { useStanze }                            from "../context/UseProvider.tsx";


const Panoramica = () => {
    const gridRef = useRef<AgGridReact>(null);
    const [ rowData, setRowData ] = useState<IStanza[]>([]);
    const [ colDefs ] = useState<ColDef<IStanza>[]>([
        {field: "id"},
        {field: "fascicolo"},
        {field: "piano"},
        {field: "id_spazio"},
        {field: "stanza"},
        {field: "destinazione_uso"},
        {field: "altezza"},
        {field: "spessore_muro"},
        {field: "riscaldamento"},
        {field: "raffrescamento"},
        {field: "illuminazione"}
    ]);
    const defaultColDef: ColDef = useMemo(() => {
        return {
            flex     : 1,
            resizable: false
        };
    }, []);
    const stanze = useStanze();


    useEffect(() => {
        setRowData(stanze.data);
    }, [ stanze.data ]);

    return (<div className="flex items-center justify-center h-full w-screen">
        <div className="ag-theme-quartz"
             style={ {
                 height: "100%",
                 width : "100%",
                 margin: "10px 0"
             } }>
            <AgGridReact
                ref={ gridRef }
                rowData={ rowData }
                columnDefs={ colDefs }
                defaultColDef={ defaultColDef }
                onFilterModified={ () => {
                } }
            />
        </div>
    </div>);
};

export default Panoramica;