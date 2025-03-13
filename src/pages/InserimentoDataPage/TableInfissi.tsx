import { AgGridReact }                                                          from "ag-grid-react";
import { useEffect, useMemo, useState }                                         from "react";
import { AllCommunityModule, ColDef, ModuleRegistry, provideGlobalGridOptions } from "ag-grid-community";
import "ag-grid-community/styles/ag-grid.css";
import "ag-grid-community/styles/ag-theme-quartz.css";
import {
    useInfissi
}                                                                               from "../../context/InfissiProvider.tsx";
import { IInfisso }                                                             from "../../models/models.tsx";

// Register all community features
ModuleRegistry.registerModules([ AllCommunityModule ]);

// Mark all grids as using legacy themes
provideGlobalGridOptions({theme: "legacy"});

const TableInfissi = () => {
    const [ rowData, setRowData ] = useState<IInfisso[]>([]);
    const [ colDefs ]             = useState<ColDef<IInfisso>[]>([
        {field: "id"}, {field: "altezza"}, {field: "larghezza"}, {field: "materiale"}, {field: "vetro"}
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
                rowData={ rowData }
                columnDefs={ colDefs }
                defaultColDef={ defaultColDef }
            />
        </div>
    </div>);
};

export default TableInfissi;