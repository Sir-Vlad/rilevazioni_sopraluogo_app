import {CardDataGrid} from "@/components/card-data-grid.tsx";
import TitlePage from "@/components/title-page.tsx";
import {Button} from "@/components/ui/button";
import {DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger} from "@/components/ui/dropdown-menu";
import {useStanze} from "@/context/UseProvider.tsx";
import {useSkipper} from "@/hooks/use-skipper.tsx";
import {IStanza} from "@/models/models.tsx";
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
    SortingState,
    useReactTable
} from "@tanstack/react-table";
import {ChevronDown} from "lucide-react";
import {useCallback, useMemo, useState} from "react";

const Panoramica = () => {
    const stanzeContext = useStanze();
    const [sorting, setSorting] = useState<SortingState>([]);
    const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);

    // @ts-ignore
    const [autoResetPageIndex, skipAutoResetPageIndex] = useSkipper();

    // fixme: Quando vengono aperti i filtri i dati vengono cancellati dalla tabella
    const columns: ColumnDef<IStanza>[] = [
        {
            accessorKey       : "cod_stanza",
            header            : "Stanza",
            enableColumnFilter: false,
            meta              : {
                filterVariant: "select"
            }
        }, {
            accessorKey       : "edificio_id",
            header            : "Chiave",
            enableColumnFilter: false,
            meta              : {
                filterVariant: "select"
            }
        }, {
            accessorKey       : "destinazione_uso",
            header            : "Destinazione Uso",
            enableColumnFilter: false
        }, {
            accessorKey       : "piano",
            header            : "Piano",
            enableColumnFilter: false
        }, {
            accessorKey       : "altezza",
            header            : "Altezza",
            enableColumnFilter: false,
            meta              : {
                filterVariant: "range"
            }
        }, {
            accessorKey       : "spessore_muro",
            header            : "Spessore Muro",
            enableColumnFilter: false
        }, {
            accessorKey       : "riscaldamento",
            header            : "Riscaldamento",
            enableColumnFilter: false,
            meta              : {
                filterVariant: "select"
            }
        }, {
            accessorKey       : "raffrescamento",
            header            : "Raffrescamento",
            enableColumnFilter: false,
            meta              : {
                filterVariant: "select"
            }
        }, {
            accessorKey       : "illuminazione",
            header            : "Illuminazione",
            enableColumnFilter: false,
            meta              : {
                filterVariant: "select"
            }
        }, {
            accessorKey       : "infissi",
            header            : "Infissi",
            cell              : props => {
                const values: string[] = props.row.getValue("infissi");
                if (!values) return null;

                const valueCounts: Record<string, number> = {};
                values.forEach((value) => {
                    valueCounts[value] = (
                                             valueCounts[value] || 0) + 1;
                });
                const uniqueValues = Object.keys(valueCounts);
                return (
                    <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button variant="ghost" className="p-0 m-0">
                        ({values.length})
                        <ChevronDown className="ml-2 h-4 w-4"/>
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    {uniqueValues.map(value => (
                        <DropdownMenuItem key={value}>
                        {value} ({valueCounts[value]})
                    </DropdownMenuItem>))}
                </DropdownMenuContent>
            </DropdownMenu>);
            },
            enableColumnFilter: false
        }
    ];

    const orderingStanze = useCallback((stanze: IStanza[]) => {
        const parsePiano = (piano: string): number => {
            if (piano === "T") return 0; // Assegna "T" a una posizione definita
            const parsed = parseInt(piano, 10);
            return isNaN(parsed) ? Number.MAX_SAFE_INTEGER : parsed; // Gestisce valori non numerici
        };

        return [...stanze].sort((a, b) => {
            const pianoA = parsePiano(a.piano);
            const pianoB = parsePiano(b.piano);
            if (pianoA !== pianoB) {
                return pianoA - pianoB;
            }

            const codStanzaA = a.cod_stanza;
            const codStanzaB = b.cod_stanza;
            return codStanzaA.localeCompare(codStanzaB);
        });
    }, []);
    const stanzeOrdinate = useMemo(() => {
        return orderingStanze(stanzeContext.data);
    }, [orderingStanze, stanzeContext.data]);

    const table = useReactTable({
        data                  : stanzeOrdinate,
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
            pagination: {pageSize: 17}
        },
        state                 : {
            sorting      : sorting,
            columnFilters: columnFilters
        },
        autoResetPageIndex    : autoResetPageIndex
    });

    return <div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col gap-2">
            <div className="flex flex-col gap-3 py-4 md:gap-4 md:py-6">
                <div className="flex flex-row justify-start items-center px-7 gap-5 mb-4">
                    <TitlePage title={"Visualizzazione Stanze"}/>
                </div>
                <div className="px-7">
                    <CardDataGrid table={table}/>
                </div>
            </div>
        </div>
    </div>;
};

export default Panoramica;
