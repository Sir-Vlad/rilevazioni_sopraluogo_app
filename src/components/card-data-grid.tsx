import { Card, CardContent, CardHeader }                                 from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";
import { Column, flexRender, RowData, Table as ReactTable }              from "@tanstack/react-table";
import { Button }                                                        from "@/components/ui/button.tsx";
import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight }        from "lucide-react";
import { Input }                                                         from "@/components/ui/input";

interface CardDataGridProps<TData> {
    table: ReactTable<TData>
    title?: string,
}

// todo: https://tanstack.com/table/latest/docs/framework/react/examples/editable-data?panel=code

function CardDataGrid<TData>({
                                 table,
                                 title
                             }: Readonly<CardDataGridProps<TData>>) {
    return <Card className="@container/card h-full py-5">
        <CardHeader>
            <h1 className="text-2xl font-bold text-primary tracking-tight">{ title }</h1>
        </CardHeader>
        <CardContent>
            <div className="flex flex-col gap-3">
                <div className="flex flex-row justify-between gap-3 w-full">
                    { table.getAllColumns()
                           .filter(column => column.getCanFilter())
                           .map(column => {
                               const columnId = column.id;

                               return (
                                   <div key={ columnId } className="w-full">
                                       <Input
                                           placeholder={ `Filtra ${ getColumnTitle(column) }...` }
                                           value={ (column.getFilterValue() as string) ?? "" }
                                           onChange={ event => column.setFilterValue(event.target.value) }
                                           className="max-w-xs"
                                       />
                                   </div>
                               );
                           }) }
                </div>
                <div className="rounded-md border">
                    <Table>
                        <TableHeader>
                            { table.getHeaderGroups().map((headerGroup) => (<TableRow key={ headerGroup.id }>
                                { headerGroup.headers.map((header) => {
                                    return (<TableHead key={ header.id } className="text-center font-bold">
                                        { header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext()) }
                                    </TableHead>);
                                }) }
                            </TableRow>)) }
                        </TableHeader>
                        <TableBody>
                            { table.getRowModel().rows?.length ? (table.getRowModel().rows.map((row) => (<TableRow
                                key={ row.id }
                                data-state={ row.getIsSelected() && "selected" }
                            >
                                { row.getVisibleCells()
                                     .map((cell) => (<TableCell key={ cell.id } className="text-center">
                                         { flexRender(cell.column.columnDef.cell, cell.getContext()) }
                                     </TableCell>)) }
                            </TableRow>))) : (<TableRow>
                                <TableCell colSpan={ table.getAllColumns().length } className="h-24 text-center">
                                    No results.
                                </TableCell>
                            </TableRow>) }
                        </TableBody>
                    </Table>
                </div>
                <div className="flex items-center justify-end space-x-3 pt-3">
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={ () => table.firstPage() }
                        disabled={ !table.getCanPreviousPage() }
                    >
                        <ChevronsLeft />
                    </Button>
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={ () => table.previousPage() }
                        disabled={ !table.getCanPreviousPage() }
                    >
                        <ChevronLeft />
                    </Button>
                    <div className="flex items-center gap-1 text-sm">
                        <p>Pagina{ " " }
                            <span className="font-bold">{ table.getState().pagination.pageIndex + 1 }</span>
                            { " " }di{ " " }
                            <span className="font-bold">{ table.getPageCount().toLocaleString() }</span>
                        </p>
                    </div>
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={ () => table.nextPage() }
                        disabled={ !table.getCanNextPage() }
                    >
                        <ChevronRight />
                    </Button>
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={ () => table.lastPage() }
                        disabled={ !table.getCanNextPage() }
                    >
                        <ChevronsRight />
                    </Button>
                </div>
            </div>
        </CardContent>
    </Card>;
}

export default CardDataGrid;

function getColumnTitle<TData extends RowData, TValue>(column: Column<TData, TValue>): string {
    const header = column.columnDef.header;

    if (typeof header === "string") {
        return header;
    }

    if (typeof header === "function") {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        if (column.columnDef.meta?.title) {
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-expect-error
            // eslint-disable-next-line @typescript-eslint/no-unsafe-return
            return column.columnDef.meta.title;
        }
    }

    const id = column.id;
    return id
        .replace(/([A-Z])/g, " $1") // Inserisce spazi prima delle lettere maiuscole
        .replace(/^./, str => str.toUpperCase()) // Rende maiuscola la prima lettera
        .trim();
};
