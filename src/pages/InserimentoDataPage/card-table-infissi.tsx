import { useInfissi, useTypes } from "@/context/UseProvider.tsx";
import {
    ColumnDef,
    ColumnFiltersState,
    getCoreRowModel,
    getFacetedMinMaxValues,
    getFacetedRowModel,
    getFacetedUniqueValues,
    getFilteredRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    RowData,
    SortingState,
    useReactTable
} from "@tanstack/react-table";
import { IInfisso } from "@/models/models.tsx";
import { Button } from "@/components/ui/button.tsx";
import { useState } from "react";
import { CheckIcon, PencilIcon, XIcon } from "lucide-react";
import { CardDataGrid, InsertCell, SelectCell } from "@/components/card-data-grid.tsx";
import { useSkipper } from "@/hooks/use-skipper.tsx";

declare module "@tanstack/react-table" {
    interface TableMeta<TData extends RowData> {
        updateData: (rowIndex: number, columnId: string, value: unknown) => void;
    }

    interface ColumnMeta<TData extends RowData, TValue> {
        editable?: boolean;
        filterVariant?: "text" | "range" | "select";
    }
}

const CardTableInfissi = () => {
    const [ editingRow, setEditingRow ] = useState<number | null>(null);
    const [ editedData, setEditedData ] = useState<Partial<IInfisso> | null>(null);

    const [ sorting, setSorting ] = useState<SortingState>([]);
    const [ columnFilters, setColumnFilters ] = useState<ColumnFiltersState>([]);

    const [ autoResetPageIndex, skipAutoResetPageIndex ] = useSkipper();

    const infissi = useInfissi();
    const {
        materialiInfissiType,
        vetroInfissiType,
        tipoInfissi
    } = useTypes();


    const handleEdit = (rowIndex: number, rowData: IInfisso) => {
        setEditingRow(rowIndex);
        setEditedData(rowData);
    };

    const handleSave = async (rowIndex: number, dataToSave: Partial<IInfisso> | null) => {
        try {
            skipAutoResetPageIndex();
            if (dataToSave) {
                await infissi.modifyInfisso(dataToSave);
                const newData = [ ...infissi.data ];
                newData[rowIndex] = { ...newData[rowIndex], ...dataToSave };
                setEditedData(null);
                setEditingRow(null);
            }
        } catch (error) {
            console.error("Errore durante il salvataggio", error);
        }
    };

    const columns: ColumnDef<IInfisso> [] = [ {
        accessorKey: "id",
        header     : "ID",
        meta       : {
            editable: false,
            filterVariant: "text"
        }
    }, {
        accessorKey: "altezza",
        header     : "Altezza",
        cell       : (cell) => {
            return InsertCell({
                ...cell,
                editingRow,
                editedData,
                setEditedData
            });
        },
        meta       : {
            filterVariant: "range"
        }
    }, {
        accessorKey: "larghezza",
        header     : "Larghezza",
        cell       : (cell) => {
            return InsertCell({
                ...cell,
                editingRow,
                editedData,
                setEditedData
            });
        },
        meta       : {
            filterVariant: "range"
        }
    }, {
        accessorKey: "materiale",
        header     : "Materiale",
        cell       : (cell) => SelectCell({
            ...cell,
            editingRow,
            editedData,
            setEditedData,
            options: materialiInfissiType
        }),
        meta       : {
            filterVariant: "select"
        }
    }, {
        accessorKey: "vetro",
        header     : "Vetro",
        cell       : (cell) => SelectCell({
            ...cell,
            editingRow,
            editedData,
            setEditedData,
            options: vetroInfissiType
        }),
        meta       : {
            filterVariant: "select"
        }
    }, {
        accessorKey: "tipo",
        header     : "Tipo",
        cell       : (cell) => SelectCell({
            ...cell,
            editingRow,
            editedData,
            setEditedData,
            options: tipoInfissi
        }),
        meta       : {
            filterVariant: "select"
        }
    }, {
        header: "Azioni",
        cell  : ({ row }) => {
            const isEditing = editingRow === row.index;

            return isEditing ? (<>
                <Button key={ `save-btn-${ row.index }` }
                        type="button" variant="ghost" size="sm"
                        onClick={ () => {
                            handleSave(row.index, editedData)
                                .then()
                                .catch(console.error);
                        } }>
                    <CheckIcon/>
                </Button>
                <Button key={ `annulla-btn-${ row.index }` } type="button" variant="ghost" size="sm"
                        onClick={ () => setEditingRow(null) }>
                    <XIcon/>
                </Button>
            </>) : (<Button key={ `edit-btn-${ row.index }` } type="button" variant="ghost" size="sm"
                            onClick={ () => {
                                handleEdit(row.index, row.original);
                            } }>
                <PencilIcon/>
            </Button>);
        }
    } ];

    const table = useReactTable({
        data                  : infissi.data,
        columns               : columns,
        getCoreRowModel       : getCoreRowModel(),
        getPaginationRowModel : getPaginationRowModel(),
        onSortingChange       : setSorting,
        getSortedRowModel     : getSortedRowModel(),
        onColumnFiltersChange : setColumnFilters,
        getFilteredRowModel   : getFilteredRowModel(),
        getFacetedRowModel    : getFacetedRowModel(),
        getFacetedUniqueValues: getFacetedUniqueValues(),
        getFacetedMinMaxValues: getFacetedMinMaxValues(),
        initialState          : {
            pagination: { pageSize: 5 }
        },
        state                 : {
            sorting      : sorting,
            columnFilters: columnFilters
        },
        autoResetPageIndex    : autoResetPageIndex,
    });

    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-1 gap-0
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-5 h-full">
        <CardDataGrid table={ table } title={ "Visualizzazione Infissi" }/>
    </div>;
};

export default CardTableInfissi;