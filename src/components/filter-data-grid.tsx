import { Column } from "@tanstack/react-table";
import { useMemo } from "react";
import ClearableSelect from "@/components/clearable-select.tsx";
import { Combobox } from "@/components/combobox.tsx";
import { DebouncedInput } from "@/components/card-data-grid.tsx";


export function FilterDataGrid({ column }: Readonly<{ column: Column<any, unknown> }>) {
    const { filterVariant } = column.columnDef.meta ?? {};

    const columnFilterValue = column.getFilterValue();

    const sortedUniqueValues: string[] = useMemo(() => {
        console.log("recomputing unique values")
        if (filterVariant === "range") return [] as string[]
        return Array.from(column.getFacetedUniqueValues().keys())
            .filter(value => value !== undefined && value !== null)
            .slice(0, 1000) as string[]
    }, [column, filterVariant]);

    if (filterVariant === "range") {
        const minValue = column.getFacetedMinMaxValues()?.[0]
        const maxValue = column.getFacetedMinMaxValues()?.[1]
        return <div>
            <div className="flex space-x-2">
                <DebouncedInput
                    type="number"
                    min={ Number(minValue ?? "") }
                    max={ Number(maxValue ?? "") }
                    value={ (columnFilterValue as [ number, number ])?.[0] ?? "" }
                    onChange={ value => column.setFilterValue((old: [ number, number ]) => [ value, old?.[1] ]) }
                    placeholder={ `Min ${ minValue ? `(${ minValue })` : "" }` }
                    className="w-full border shadow rounded"
                />
                <DebouncedInput
                    type="number"
                    min={ Number(minValue ?? "") }
                    max={ Number(maxValue ?? "") }
                    value={ (columnFilterValue as [ number, number ])?.[1] ?? "" }
                    onChange={ value => column.setFilterValue((old: [ number, number ]) => [ old?.[0], value ]) }
                    placeholder={ `Max ${ maxValue ? `(${ maxValue })` : "" }` }
                    className="w-full border shadow rounded"
                />
            </div>
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
            <Combobox options={ sortedUniqueValues }
                      value={ columnFilterValue?.toString() ?? "" }
                      onChange={ (value) => column.setFilterValue(value) }/>
        </>;
    }
}