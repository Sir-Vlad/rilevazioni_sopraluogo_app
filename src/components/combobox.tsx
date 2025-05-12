"use client";

import { useState } from "react";
import { Check, ChevronsUpDown } from "lucide-react";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from "@/components/ui/command";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";

interface ComboboxProps {
    options: string[],
    value: string,
    onChange: (value: string) => void,
    placeholder?: string,
}

export function Combobox({
                             options,
                             value,
                             onChange,
                             placeholder = "Seleziona una opzione"
                         }: Readonly<ComboboxProps>) {
    const [ open, setOpen ] = useState(false);

    return (<Popover open={ open } onOpenChange={ setOpen }>
        <PopoverTrigger asChild>
            <Button
                variant="outline"
                role="combobox"
                aria-expanded={ open }
                className="w-full justify-between"
            >
                { value ? options.find((valueOpt) => value === valueOpt) : placeholder }
                <ChevronsUpDown className="opacity-50"/>
            </Button>
        </PopoverTrigger>
        <PopoverContent className="w-full p-0">
            <Command>
                <CommandInput placeholder={ placeholder } className="h-9"/>
                <CommandList>
                    <CommandEmpty>No data found.</CommandEmpty>
                    <CommandGroup>
                        <CommandItem onSelect={ () => onChange( "" ) }>
                            Reset filter
                        </CommandItem>
                        { options.map((valueOpt) => (<CommandItem
                            key={ valueOpt }
                            value={ valueOpt }
                            onSelect={ (currentValue) => {
                                onChange(currentValue !== valueOpt ? "" : currentValue);
                                setOpen(false);
                            } }
                        >
                            { valueOpt }
                            <Check
                                className={ cn("ml-auto", value === valueOpt ? "opacity-100" : "opacity-0") }
                            />
                        </CommandItem>)) }
                    </CommandGroup>
                </CommandList>
            </Command>
        </PopoverContent>
    </Popover>);
}
