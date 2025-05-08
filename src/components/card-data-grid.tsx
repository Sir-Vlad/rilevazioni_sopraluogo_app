import { Card, CardContent, CardHeader }                                                      from "@/components/ui/card.tsx";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
}                                                                                             from "@/components/ui/table.tsx";
import {
    Column,
    flexRender,
    RowData,
    Table as ReactTable
}                                                                                             from "@tanstack/react-table";
import {
    Button
}                                                                                             from "@/components/ui/button.tsx";
import { ArrowDown, ArrowUp, ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight, Funnel } from "lucide-react";
import {
    Input
}                                                                                             from "@/components/ui/input";
import TitleCard
                                                                                              from "@/components/title-card.tsx";
import { InputHTMLAttributes, useEffect, useMemo, useState }                                  from "react";
import ClearableSelect
                                                                                              from "@/components/clearable-select.tsx";
import {
    Popover,
    PopoverContent,
    PopoverTrigger
}                                                                                             from "@/components/ui/popover.tsx";
import {
    Label
}                                                                                             from "@/components/ui/label.tsx";
import { Combobox }                                                                           from "./combobox";

interface CardDataGridProps<TData> {
    table: ReactTable<TData>
    title?: string,
}

// https://tanstack.com/table/latest/docs/framework/react/examples/editable-data?panel=code

function CardDataGrid<TData>({
                                 table,
                                 title
                             }: Readonly<CardDataGridProps<TData>>) {
    return <Card className="@container/card h-full py-5">
        <CardHeader>
            <TitleCard title={ title ?? "Dati" } />
        </CardHeader>
        <CardContent>
            <div className="flex flex-col gap-3">
                <div className="flex items-end justify-end">
                    <Popover>
                        <PopoverTrigger asChild>
                            <Button variant={ "outline" }>
                                <Funnel />
                            </Button>
                        </PopoverTrigger>
                        <PopoverContent side={ "left" }>
                            <div className="grid gap-4">
                                <div className="space-y-2">
                                    <h4 className="font-medium leading-none">Filtri</h4>
                                </div>
                                <div className="grid gap-2">
                                    { table.getHeaderGroups().map((headerGroup) => headerGroup.headers.map(
                                        header => header.column.getCanFilter() ? (
                                            <div key={ header.id } className="flex flex-col gap-2">
                                                <Label htmlFor={ header.column.id }>{ getColumnTitle(
                                                    header.column) }</Label>
                                                <Filter column={ header.column } />
                                            </div>) : null)) }
                                </div>
                            </div>
                        </PopoverContent>
                    </Popover>
                </div>
                <div className="rounded-md border">
                    <Table>
                        <TableHeader>
                            { table.getHeaderGroups().map((headerGroup) => (<TableRow key={ headerGroup.id }>
                                { headerGroup.headers.map((header) => {
                                    return (<TableHead key={ header.id } className="text-center font-bold">
                                        { header.isPlaceholder ? null : (<div
                                            className={ `flex gap-2 items-center justify-center
                                                ${ header.column.getCanSort() ? "cursor-pointer select-none" : "" }` }
                                            onClick={ header.column.getToggleSortingHandler() }
                                            role={ "button" }
                                        >
                                            { flexRender(header.column.columnDef.header, header.getContext()) }
                                            { {
                                                asc : <ArrowUp size={ "1rem" } />,
                                                desc: <ArrowDown size={ "1rem" } />
                                            }[header.column.getIsSorted() as string] ?? null }
                                        </div>) }
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
                                     .map((cell) => (<TableCell key={ cell.id } className="justify-center text-center">
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
        if (column.columnDef.meta?.title) {
            return column.columnDef.meta.title;
        }
    }

    const id = column.id;
    return id
        .replace(/([A-Z])/g, " $1") // Inserisce spazi prima delle lettere maiuscole
        .replace(/^./, str => str.toUpperCase()) // Rende maiuscola la prima lettera
        .trim();
}

function Filter({column}: Readonly<{ column: Column<any, unknown> }>) {
    const {filterVariant} = column.columnDef.meta ?? {};

    const columnFilterValue = column.getFilterValue();

    const sortedUniqueValues = useMemo(
        () => filterVariant === "range" ? [] : Array.from(column.getFacetedUniqueValues().keys())
                                                    .slice(0, 5000), [ column, filterVariant ]);

    if (filterVariant === "range") {
        return <div>
            <div className="flex space-x-2">
                <DebouncedInput
                    type="number"
                    min={ Number(column.getFacetedMinMaxValues()?.[0] ?? "") }
                    max={ Number(column.getFacetedMinMaxValues()?.[1] ?? "") }
                    value={ (columnFilterValue as [ number, number ])?.[0] ?? "" }
                    onChange={ value => column.setFilterValue((old: [ number, number ]) => [ value, old?.[1] ]) }
                    placeholder={ `Min ${ column.getFacetedMinMaxValues()?.[0] ? `(${ column.getFacetedMinMaxValues()?.[0] })` : "" }` }
                    className="w-full border shadow rounded"
                />
                <DebouncedInput
                    type="number"
                    min={ Number(column.getFacetedMinMaxValues()?.[0] ?? "") }
                    max={ Number(column.getFacetedMinMaxValues()?.[1] ?? "") }
                    value={ (columnFilterValue as [ number, number ])?.[1] ?? "" }
                    onChange={ value => column.setFilterValue((old: [ number, number ]) => [ old?.[0], value ]) }
                    placeholder={ `Max ${ column.getFacetedMinMaxValues()?.[1] ? `(${ column.getFacetedMinMaxValues()?.[1] })` : "" }` }
                    className="w-full border shadow rounded"
                />
            </div>
            <div className="h-1" />
        </div>;
    } else if (filterVariant === "select") {
        return <ClearableSelect onChange={ value => column.setFilterValue(value) }
                                value={ columnFilterValue?.toString() ?? "" }
                                options={ sortedUniqueValues }
                                onClear={ () => column.setFilterValue(undefined) }
        />;
    } else {
        return <>
            {/* Autocomplete suggestions from faceted values feature */ }
            <Combobox options={ sortedUniqueValues } />
            <DebouncedInput
                type="text"
                value={ (columnFilterValue ?? "") as string }
                onChange={ value => column.setFilterValue(value) }
                placeholder={ `Search... (${ column.getFacetedUniqueValues().size })` }
                className="w-36 border shadow rounded"
                list={ column.id + "list" }
            />
            <div className="h-1" />
        </>;
    }
}

// A typical debounced input react component
function DebouncedInput({
                            value: initialValue,
                            onChange,
                            debounce = 500,
                            ...props
                        }: {
    value: string | number
    onChange: (value: string | number) => void
    debounce?: number
} & Omit<InputHTMLAttributes<HTMLInputElement>, "onChange">) {
    const [ value, setValue ] = useState(initialValue);

    useEffect(() => {
        setValue(initialValue);
    }, [ initialValue ]);

    useEffect(() => {
        const timeout = setTimeout(() => {
            onChange(value);
        }, debounce);

        return () => clearTimeout(timeout);
    }, [ debounce, onChange, value ]);

    return (<Input { ...props } value={ value } onChange={ e => setValue(e.target.value) } />);
}