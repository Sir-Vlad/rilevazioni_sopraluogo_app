import { AgGridReact }                          from "ag-grid-react";
import { useEffect, useMemo, useRef, useState } from "react";
import {
    AllCommunityModule,
    ClientSideRowModelModule,
    ColDef,
    ModuleRegistry,
    NumberFilterModule,
    provideGlobalGridOptions,
    ValidationModule
}                                               from "ag-grid-community";
import "ag-grid-community/styles/ag-grid.css";
import "ag-grid-community/styles/ag-theme-quartz.css";
import { useInfissi }                           from "../../context/InfissiProvider.tsx";
import { IInfisso }                             from "../../models/models.tsx";

// Register all community features
ModuleRegistry.registerModules([ AllCommunityModule, ClientSideRowModelModule, NumberFilterModule, ValidationModule ]);

// Mark all grids as using legacy themes
provideGlobalGridOptions({theme: "legacy"});

const TableInfissi = () => {
    const gridRef                 = useRef<AgGridReact>(null);
    const [ rowData, setRowData ] = useState<IInfisso[]>([]);
    const [ colDefs ]             = useState<ColDef<IInfisso>[]>([
        {
            field         : "id",
            filter        : "agTextColumnFilter",
            floatingFilter: true,
            filterParams  : {
                buttons: [ "reset" ]
            }
        }, {
            field         : "altezza",
            filter        : "agNumberColumnFilter",
            floatingFilter: true,
            filterParams  : {
                buttons: [ "reset" ]
            }
        }, {
            field         : "larghezza",
            filter        : "agNumberColumnFilter",
            floatingFilter: true,
            filterParams  : {
                buttons: [ "reset" ]
            }
        }, {field: "materiale"}, {field: "vetro"},
        {field: "tipo"}
    ]);
    const infissi                 = useInfissi();


    useEffect(() => {
        setRowData(infissi.data);
    }, [ infissi.data ]);

    const defaultColDef: ColDef = useMemo(() => {
        return {
            flex     : 1,
            resizable: false
        };
    }, []);

    return (<div className="h-full w-full">
        <h2 className="text-2xl font-bold text-gray-800 mb-6 border-b pb-3">
            Visualizzazione Infissi
        </h2>
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

export default TableInfissi;