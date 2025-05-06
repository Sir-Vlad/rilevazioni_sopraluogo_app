// InfissiSelect.tsx
import { Fragment, useState } from "react";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectSeparator,
    SelectTrigger,
    SelectValue
}                             from "@/components/ui/select.tsx";
import {
    Button
}                             from "@/components/ui/button.tsx";

interface InfissiSelectProps {
    options: string[]; // Lista delle opzioni da visualizzare
    value: string; // Valore selezionato
    onChange: (value: string) => void; // Funzione callback per gestione selezione
    placeholder?: string; // Placeholder per il valore di default
    onClear?: () => void; // Funzione opzionale per gestire il reset
}

const ClearableSelect = ({
                             options,
                             value,
                             onChange,
                             placeholder = "Seleziona un'opzione",
                             onClear
                         }: Readonly<InfissiSelectProps>) => {
    const [ open, setOpen ] = useState(false);

    return (<Select onValueChange={ onChange } value={ value } onOpenChange={ setOpen } open={ open }>
        <SelectTrigger className="w-full">
            <SelectValue placeholder={ placeholder } />
        </SelectTrigger>
        <SelectContent>
            { onClear && (<>
                <Button
                    className="w-full px-2"
                    variant="secondary"
                    size="sm"
                    onClick={ (e) => {
                        e.stopPropagation();
                        onClear();
                        setOpen(false);
                    } }
                >
                    Clear
                </Button>
                <SelectSeparator />
            </>) }
            { options.map((option, index) => (<Fragment key={ index }>
                <SelectItem value={ option }>{ option }</SelectItem>
            </Fragment>)) }
        </SelectContent>
    </Select>);
};

export default ClearableSelect;