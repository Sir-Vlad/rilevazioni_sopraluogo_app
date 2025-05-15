import { Search } from "lucide-react";

import { Label } from "@/components/ui/label";
import { SidebarInput } from "@/components/ui/sidebar";
import * as React from "react";
import { Dispatch, SetStateAction } from "react";

export function SearchForm({
                               onSearch,
                               ...props
                           }: { onSearch: Dispatch<SetStateAction<string | null>> } & React.ComponentProps<"form">) {
    return (<form { ...props }>
            <div className="relative p-2">
                <Label htmlFor="search" className="sr-only">
                    Search
                </Label>
                <SidebarInput
                    id="search"
                    placeholder="Type to search..."
                    className="h-8 pl-7"
                    onChange={ e => {
                        const { value } = e.target;
                        if (value === "") {
                            onSearch(null);
                            return;
                        }                        
                        onSearch(value);
                    } }
                />
                <Search
                    className="pointer-events-none absolute top-1/2 left-4 size-4 -translate-y-1/2 opacity-50 select-none"/>
            </div>
        </form>);
}
