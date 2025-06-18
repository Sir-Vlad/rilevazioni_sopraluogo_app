import { Card, CardContent, CardHeader } from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";
import { CellContext, Column, flexRender, RowData, Table as ReactTable } from "@tanstack/react-table";
import { Button } from "@/components/ui/button.tsx";
import { ArrowDown, ArrowUp, ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight, Funnel } from "lucide-react";
import { Input } from "@/components/ui/input";
import TitleCard from "@/components/title-card.tsx";
import { Dispatch, InputHTMLAttributes, SetStateAction, useCallback, useEffect, useState } from "react";
import ClearableSelect from "@/components/clearable-select.tsx";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover.tsx";
import { Label } from "@/components/ui/label.tsx";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { IInfisso } from "@/models/models.tsx";
import { handleInputNumericChange } from "@/helpers/helpers.ts";
import { debounce } from "lodash";
import { FilterDataGrid } from "@/components/filter-data-grid.tsx";

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
        { title && <CardHeader>
            <TitleCard title={ title }/>
        </CardHeader> }
        <CardContent>
            <div className="flex flex-col gap-3">
                <div className="flex items-end justify-end">
                    <ScrollArea>
                        <Popover>
                            <PopoverTrigger asChild>
                                <Button variant={ "outline" }>
                                    <Funnel/>
                                </Button>
                            </PopoverTrigger>
                            <PopoverContent side={ "left" }>
                                <div className="grid gap-4">
                                    <div className="flex justify-between items-center space-y-2">
                                        <h4 className="font-medium leading-none">Filtri</h4>
                                        <Button variant="outline"
                                                size="sm"
                                                onClick={ () => table.resetColumnFilters(true) }
                                        >
                                            Reset Filtri
                                        </Button>
                                    </div>
                                    <div className="grid gap-2">
                                        { table.getHeaderGroups().map((headerGroup) => headerGroup.headers.map(header => header.column.getCanFilter() ? (
                                            <div key={ header.id } className="flex flex-col gap-3">
                                                <div className="flex flex-col gap-2">
                                                    <Label
                                                        htmlFor={ header.column.id }>{ getColumnTitle(header.column) }</Label>
                                                    <FilterDataGrid column={ header.column }/>
                                                </div>
                                            </div>) : null)) }
                                    </div>
                                </div>
                            </PopoverContent>
                        </Popover>
                    </ScrollArea>
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
                                            onClick={ () => header.column.getToggleSortingHandler() }
                                        >
                                            { flexRender(header.column.columnDef.header, header.getContext()) }
                                            { {
                                                asc : <ArrowUp size={ "1rem" }/>,
                                                desc: <ArrowDown size={ "1rem" }/>
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
                        <ChevronsLeft/>
                    </Button>
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={ () => table.previousPage() }
                        disabled={ !table.getCanPreviousPage() }
                    >
                        <ChevronLeft/>
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
                        <ChevronRight/>
                    </Button>
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={ () => table.lastPage() }
                        disabled={ !table.getCanNextPage() }
                    >
                        <ChevronsRight/>
                    </Button>
                </div>
            </div>
        </CardContent>
    </Card>;
}

function getColumnTitle<TData extends RowData, TValue>(column: Column<TData, TValue>): string {
    const header = column.columnDef.header;

    if (typeof header === "string") {
        return header;
    }

    const id = column.id;
    return id
        .replace(/([A-Z])/g, " $1") // Inserisce spazi prima delle lettere maiuscole
        .replace(/^./, str => str.toUpperCase()) // Rende maiuscola la prima lettera
        .trim();
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

    return (<Input { ...props } value={ value } onChange={ e => setValue(e.target.value) }/>);
}

const useEditingCellState = <TData, >(editingRow: number | null, rowIndex: number, columnId: string, editedData: Partial<TData> | null, getValue: () => unknown, setEditedData: Dispatch<SetStateAction<Partial<TData> | null>>) => {
    const isEditing = editingRow === rowIndex;
    const [ localValue, setLocalValue ] = useState(() => {
        return isEditing ? editedData?.[columnId as keyof TData] ?? getValue() : getValue();
    });

    // eslint-disable-next-line react-hooks/exhaustive-deps
    const updateEditingData = useCallback(debounce((newValue) => {
        setEditedData((prevState) => ({
            ...prevState,
            [columnId]: newValue as IInfisso[keyof IInfisso]
        }));
    }, 300), [ setEditedData ]);

    return {
        isEditing,
        localValue,
        setLocalValue,
        updateEditingData
    };
};

const NonEditableCell = ({ value }: { value: string }) => <span>{ value }</span>;

interface ICellProps<TData> extends CellContext<TData, unknown> {
    editingRow: number | null;
    editedData: Partial<TData> | null;
    setEditedData: Dispatch<SetStateAction<Partial<TData> | null>>;
}

const InsertCell = <TData, >({
                                 getValue,
                                 row,
                                 column,
                                 editingRow,
                                 editedData,
                                 setEditedData
                             }: ICellProps<TData>) => {
    const {
        isEditing,
        localValue,
        setLocalValue,
        updateEditingData
    } = useEditingCellState(editingRow, row.index, column.id, editedData, getValue, setEditedData);


    if (column.columnDef.meta?.editable === false) {
        return <NonEditableCell value={ getValue() as string }/>;
    }

    return <div className="flex flex-row items-center justify-center">
        { isEditing ? (<Input
            key={ `${ row.index }-${ column.id }` }
            value={ localValue as string }
            onChange={ e => {
                handleInputNumericChange(e, (value) => {
                    setLocalValue(value);
                    updateEditingData(value);
                });
            } }
            className={ "text-center" }
            style={ { "width": "4rem" } }
        />) : (<span>{ localValue as string }</span>) }
    </div>;
};

interface ISelectCellProps<TData> extends ICellProps<TData> {
    options: string[];
}

const SelectCell = <TData, >({
                                 getValue,
                                 row,
                                 column,
                                 editingRow,
                                 editedData,
                                 setEditedData,
                                 options
                             }: ISelectCellProps<TData>) => {
    const {
        isEditing,
        localValue,
        setLocalValue,
        updateEditingData
    } = useEditingCellState(editingRow, row.index, column.id, editedData, getValue, setEditedData);

    if (column.columnDef.meta?.editable === false) {
        return <NonEditableCell value={ getValue() as string }/>;
    }

    return <div className="flex flex-row items-center justify-center">
        { isEditing ? (<ClearableSelect onChange={ (value) => {
            setLocalValue(value);
            updateEditingData(value);
        } } options={ options } value={ localValue as string } className={ "w-30" }/>) : (
            <span>{ localValue as string }</span>) }
    </div>;
};

export {
    CardDataGrid, SelectCell, InsertCell, DebouncedInput
};