import { useInfissi }  from "@/context/UseProvider.tsx";
import {
    ColumnDef,
    ColumnFiltersState,
    getCoreRowModel,
    getFilteredRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    SortingState,
    useReactTable
}                      from "@tanstack/react-table";
import { IInfisso }    from "@/models/models.tsx";
import { Button }      from "@/components/ui/button.tsx";
import { useState }    from "react";
import { ArrowUpDown } from "lucide-react";
import { capitalize }  from "@/helpers/helpers.ts";
import CardDataGrid    from "@/components/card-data-grid.tsx";

const columns: ColumnDef<IInfisso> [] = [
    {
        accessorKey: "id",
        header     : ({column}) => {
            return (<Button
                variant="ghost"
                onClick={ () => column.toggleSorting(column.getIsSorted() === "asc") }
            >
                ID
                <ArrowUpDown className="ml-2 h-4 w-4" />
            </Button>);
        },
        meta       : {
            title: "ID"
        }
    }, {
        accessorKey: "altezza",
        header     : "Altezza",
        filterFn   : "includesString"
    }, {
        accessorKey: "larghezza",
        header     : "Larghezza",
        filterFn   : "includesString"
    }, {
        accessorKey       : "materiale",
        header            : "Materiale",
        enableColumnFilter: false
    }, {
        accessorKey       : "vetro",
        header            : "Vetro",
        enableColumnFilter: false
    }, {
        accessorKey       : "tipo",
        header            : "Tipo",
        cell              : props => capitalize(props.getValue() as string),
        enableColumnFilter: false
    }
];

const CardTableInfissi = () => {
    const [ sorting, setSorting ] = useState<SortingState>([]);
    const [ columnFilters, setColumnFilters ] = useState<ColumnFiltersState>([]);

    const infissi = useInfissi();
    const table = useReactTable({
        data                 : infissi.data,
        columns              : columns,
        getCoreRowModel      : getCoreRowModel(),
        getPaginationRowModel: getPaginationRowModel(),
        onSortingChange      : setSorting,
        getSortedRowModel    : getSortedRowModel(),
        onColumnFiltersChange: setColumnFilters,
        getFilteredRowModel  : getFilteredRowModel(),
        initialState         : {
            pagination: {pageSize: 5}
        },
        state                : {
            sorting      : sorting,
            columnFilters: columnFilters
        }
    });

    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-1 gap-0
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-5 h-full">
        <CardDataGrid table={ table } title={ "Visualizzazione Infissi" } />
    </div>;
};

export default CardTableInfissi;