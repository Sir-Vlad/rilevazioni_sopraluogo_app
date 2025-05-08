"use client";

import { useState }              from "react";
import { Check, ChevronsUpDown } from "lucide-react";

import { cn }                                                                          from "@/lib/utils";
import { Button }                                                                      from "@/components/ui/button";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from "@/components/ui/command";
import { Popover, PopoverContent, PopoverTrigger }                                     from "@/components/ui/popover";

export function Combobox({options}: Readonly<{ options: string[] }>) {
    const [ open, setOpen ] = useState(false);
    const [ value, setValue ] = useState("");

    return (<Popover open={ open } onOpenChange={ setOpen }>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={ open }
                    className="w-[200px] justify-between"
                >
                    { value ? options.find((valueOpt) => value === valueOpt) : "Select framework..." }
                    <ChevronsUpDown className="opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[200px] p-0">
                <Command>
                    <CommandInput placeholder="Search framework..." className="h-9" />
                    <CommandList>
                        <CommandEmpty>No framework found.</CommandEmpty>
                        <CommandGroup>
                            { options.map((valueOpt) => (<CommandItem
                                    key={ valueOpt }
                                    value={ valueOpt }
                                    onSelect={ (currentValue) => {
                                        setValue(currentValue === valueOpt ? "" : currentValue);
                                        setOpen(false);
                                    } }
                                >
                                    { valueOpt }
                                    <Check
                                        className={ cn(
                                            "ml-auto",
                                            value === valueOpt ? "opacity-100" : "opacity-0") }
                                    />
                                </CommandItem>)) }
                        </CommandGroup>
                    </CommandList>
                </Command>
            </PopoverContent>
        </Popover>);
}
