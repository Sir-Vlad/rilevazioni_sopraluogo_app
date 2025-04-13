import { Card, CardContent, CardHeader }                                 from "@/components/ui/card";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";
import { useInfissi }                                                    from "@/context/UseProvider.tsx";
import {
    ColumnDef,
    ColumnFiltersState,
    flexRender,
    getCoreRowModel,
    getFilteredRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    SortingState,
    useReactTable
}                                                                        from "@tanstack/react-table";
import { IInfisso }                                                      from "@/models/models.tsx";
import { Button }                                                        from "@/components/ui/button.tsx";
import { useState }                                                      from "react";
import { ArrowUpDown }                                                   from "lucide-react";
import { Input }                                                         from "@/components/ui/input";
import { capitalize }                                                    from "@/helpers/helpers.ts";

const columns: ColumnDef<IInfisso> [] = [
    {
        accessorKey: "id",
        header     : ({column}) => {
            return (
                <Button
                    variant="ghost"
                    onClick={ () => column.toggleSorting(column.getIsSorted() === "asc") }
                >
                    ID
                    <ArrowUpDown className="ml-2 h-4 w-4" />
                </Button>
            );
        }
    },
    {
        accessorKey: "altezza",
        header     : "Altezza",
        filterFn   : "weakEquals"
    },
    {
        accessorKey: "larghezza",
        header     : "Larghezza",
        filterFn   : "weakEquals"
    },
    {
        accessorKey: "materiale",
        header     : "Materiale"
    },
    {
        accessorKey: "vetro",
        header     : "Vetro"
    },
    {
        accessorKey: "tipo",
        header     : "Tipo",
        cell       : props => capitalize(props.getValue() as string)
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
        <Card className="@container/card h-full py-5">
            <CardHeader>
                <h1 className="text-2xl font-bold text-primary tracking-tight">Visualizzazione Infissi</h1>
            </CardHeader>
            <CardContent>
                <div className="flex flex-col">
                    <div className="flex items-center gap-5 pb-4">
                        <Input
                            placeholder="Filter ids..."
                            value={ (table.getColumn("id")?.getFilterValue() as string) ?? "" }
                            onChange={ (event) =>
                                table.getColumn("id")?.setFilterValue(event.target.value)
                            }
                        />
                        <Input
                            placeholder="Filter altezza..."
                            value={ (table.getColumn("altezza")?.getFilterValue() as string) ?? "" }
                            onChange={ (event) => {
                                console.log(table.getColumn("altezza"));
                                table.getColumn("altezza")?.setFilterValue(event.target.value);
                            }
                            }
                        />
                        <Input
                            placeholder="Filter larghezza..."
                            value={ (table.getColumn("larghezza")?.getFilterValue() as string) ?? "" }
                            onChange={ (event) =>
                                table.getColumn("larghezza")?.setFilterValue(event.target.value)
                            }
                        />
                    </div>
                    <div className="rounded-md border">
                        <Table>
                            <TableHeader>
                                { table.getHeaderGroups().map((headerGroup) => (
                                    <TableRow key={ headerGroup.id }>
                                        { headerGroup.headers.map((header) => {
                                            return (
                                                <TableHead key={ header.id } className="text-center font-bold">
                                                    { header.isPlaceholder
                                                        ? null
                                                        : flexRender(
                                                            header.column.columnDef.header,
                                                            header.getContext()
                                                        ) }
                                                </TableHead>
                                            );
                                        }) }
                                    </TableRow>
                                )) }
                            </TableHeader>
                            <TableBody>
                                { table.getRowModel().rows?.length ? (
                                    table.getRowModel().rows.map((row) => (
                                        <TableRow
                                            key={ row.id }
                                            data-state={ row.getIsSelected() && "selected" }
                                        >
                                            { row.getVisibleCells().map((cell) => (
                                                <TableCell key={ cell.id } className="text-center">
                                                    { flexRender(cell.column.columnDef.cell, cell.getContext()) }
                                                </TableCell>
                                            )) }
                                        </TableRow>
                                    ))
                                ) : (
                                    <TableRow>
                                        <TableCell colSpan={ columns.length } className="h-24 text-center">
                                            No results.
                                        </TableCell>
                                    </TableRow>
                                ) }
                            </TableBody>
                        </Table>
                    </div>
                    <div className="flex items-center justify-end space-x-3 pt-3">
                        <Button
                            variant="outline"
                            size="sm"
                            onClick={ () => table.previousPage() }
                            disabled={ !table.getCanPreviousPage() }
                        >
                            Precedente
                        </Button>
                        <Button
                            variant="outline"
                            size="sm"
                            onClick={ () => table.nextPage() }
                            disabled={ !table.getCanNextPage() }
                        >
                            Successivo
                        </Button>
                    </div>
                </div>
            </CardContent>
        </Card>
    </div>;
};

export default CardTableInfissi;