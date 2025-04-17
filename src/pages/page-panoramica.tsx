import {
    ColumnDef,
    ColumnFiltersState,
    getCoreRowModel,
    getFilteredRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    SortingState,
    useReactTable
}                               from "@tanstack/react-table";
import { IStanza }              from "@/models/models.tsx";
import { StanzeContext }        from "@/context/Context.tsx";
import { useContext, useState } from "react";
import TitlePage                from "@/components/title-page.tsx";
import CardDataGrid             from "@/components/card-data-grid.tsx";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger
}                               from "@/components/ui/dropdown-menu";
import { Button }               from "@/components/ui/button";
import { ChevronDown }          from "lucide-react";
import { useEdifici }           from "@/context/UseProvider.tsx";

const columns: ColumnDef<IStanza>[] = [
    {
        accessorKey: "stanza",
        header     : "Stanza",
        filterFn   : "includesString"
    }, {
        accessorKey: "chiave",
        header     : "Chiave",
        filterFn   : "includesString"
    }, {
        accessorKey       : "destinazione_uso",
        header            : "Destinazione Uso",
        enableColumnFilter: false
    }, {
        accessorKey       : "piano",
        header            : "Piano",
        enableColumnFilter: false
    }, {
        accessorKey: "altezza",
        header     : "Altezza",
        filterFn   : "includesString"
    }, {
        accessorKey       : "spessore_muro",
        header            : "Spessore Muro",
        enableColumnFilter: false
    }, {
        accessorKey: "riscaldamento",
        header     : "Riscaldamento"
    }, {
        accessorKey: "raffrescamento",
        header     : "Raffrescamento",
        filterFn   : "includesString"
    }, {
        accessorKey: "illuminazione",
        header     : "Illuminazione",
        filterFn   : "includesString"
    }, {
        accessorKey       : "infissi",
        header            : "Infissi",
        cell              : props => {
            const values: string[] = props.row.getValue("infissi");
            if (!values) return null;

            const valueCounts: Record<string, number> = {};
            values.forEach((value) => {
                valueCounts[value] = (valueCounts[value] || 0) + 1;
            });
            const uniqueValues = Object.keys(valueCounts);
            return (<DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button variant="ghost" className="p-0 m-0">
                        ({ values.length })
                        <ChevronDown className="ml-2 h-4 w-4" />
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    { uniqueValues.map((value, index) => (<DropdownMenuItem key={ index }>
                        { value } ({ valueCounts[value] })
                    </DropdownMenuItem>)) }
                </DropdownMenuContent>
            </DropdownMenu>);
        },
        enableColumnFilter: false
    }
];

const Panoramica = () => {
    const stanzeContext = useContext(StanzeContext);
    // todo: decidere se la visualizzazione deve essere condizionata dal context oppure avere qualcosa di locale
    const {selectedEdificio} = useEdifici();
    const [ sorting, setSorting ] = useState<SortingState>([]);
    const [ columnFilters, setColumnFilters ] = useState<ColumnFiltersState>([]);

    const table = useReactTable({
        data                 : stanzeContext!.data.filter(value => value.chiave === selectedEdificio),
        columns              : columns,
        getCoreRowModel      : getCoreRowModel(),
        getPaginationRowModel: getPaginationRowModel(),
        onSortingChange      : setSorting,
        getSortedRowModel    : getSortedRowModel(),
        onColumnFiltersChange: setColumnFilters,
        getFilteredRowModel  : getFilteredRowModel(),
        initialState         : {
            pagination: {pageSize: 17}
        },
        state                : {
            sorting      : sorting,
            columnFilters: columnFilters
        }
    });

    return <div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col gap-2">
            <div className="flex flex-col gap-3 py-4 md:gap-4 md:py-6">
                <div className="flex flex-row justify-start items-center px-7 gap-5 mb-4">
                    <TitlePage title={ "Visualizzazione Stanze" } />
                </div>
                <div className="px-7">
                    <CardDataGrid table={ table } />
                </div>
            </div>
        </div>
    </div>;
};

export default Panoramica;
