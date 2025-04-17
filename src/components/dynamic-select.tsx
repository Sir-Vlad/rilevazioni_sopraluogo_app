import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select.tsx";
import {
    FormControl
}                                                                        from "@/components/ui/form.tsx";
import {
    Button
}                                                                        from "@/components/ui/button.tsx";
import {
    EllipsisIcon,
    MinusIcon,
    PlusIcon
}                                                                        from "lucide-react";
import {
    useInfissi
}                                                                        from "@/context/UseProvider.tsx";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
}                                                                        from "@/components/ui/dialog.tsx";
import {
    Input
}                                                                        from "@/components/ui/input.tsx";
import {
    useState
}                                                                        from "react";
import {
    handleInputNumericChange
}                                                                        from "@/helpers/helpers.ts";
import {
    ScrollArea
}                                                                        from "@/components/ui/scroll-area.tsx";

interface DynamicSelectProps {
    onChange: (value: string[]) => void;
    values: string[] | undefined;
}


export default function DynamicSelect({
                                          onChange,
                                          values
                                      }: Readonly<DynamicSelectProps>) {
    const infissi = useInfissi();
    const infissiData = infissi.data.map((item) => {
        if (!item.id) {
            throw new Error("Infisso doesn't have id");
        }
        return item.id;
    });


    return <ScrollArea className="h-65 flex flex-col gap-5 p-4 rounded-lg border">
        <div className="grid grid-cols-4 gap-5">
            { (values || []).map((item, index) => {
                return <div key={ index + 1 }>
                    <Select onValueChange={ newValue => {
                        const updateValues = [ ...values || [] ];
                        updateValues[index] = newValue;
                        onChange(updateValues);
                    } } value={ item }>
                        <FormControl>
                            <SelectTrigger className="w-full">
                                <SelectValue placeholder="" />
                            </SelectTrigger>
                        </FormControl>
                        <SelectContent className="max-h-65">
                            { infissiData.map(infisso => {
                                return <SelectItem value={ infisso } key={ infisso }>{ infisso }</SelectItem>;
                            }) }
                        </SelectContent>
                    </Select>
                </div>;
            }) }
            {/* Add new value btn */ }
            <div className="flex flex-row gap-2">
                <Button variant="outline" size="icon" type="button" onClick={ () => {
                    const newValues = [ ...(values || []), "" ];
                    onChange(newValues);
                } }>
                    <PlusIcon />
                </Button>
                <Button variant="outline" size="icon" type="button"
                        disabled={ values === undefined || values.length === 0 }
                        onClick={ () => {
                            if (!values?.length) return;
                            const newValues = [ ...values ];
                            newValues.pop();
                            onChange(newValues);
                        } }>
                    <MinusIcon />
                </Button>
                <AddInfissiDialog values={ values } onChange={ onChange } infissiData={ infissiData } />
            </div>
        </div>
    </ScrollArea>;
};

function AddInfissiDialog({
                              values,
                              onChange,
                              infissiData
                          }: Readonly<{
    values: string[] | undefined, onChange: (value: string[]) => void, infissiData: string[]
}>) {
    const [ isOpen, setIsOpen ] = useState(false);
    const [ numberOfInfissi, setNumberOfInfissi ] = useState(0);
    const [ selectedType, setSelectedType ] = useState("");

    const handleAddInfissi = () => {
        // Conversione a numero
        if (isNaN(numberOfInfissi) || numberOfInfissi <= 0 || numberOfInfissi > 20) {
            alert("Inserisci un numero valido di infissi");
            return;
        }

        if (!selectedType) {
            alert("Seleziona una tipologia");
            return;
        }
        // Creare un array con il numero specificato di infissi tutti dello stesso tipo
        const newInfissi: string[] = Array(numberOfInfissi).fill(selectedType) as string[];
        // Aggiungere questi nuovi infissi all'array esistente
        const updateValues = [ ...(values || []), ...newInfissi ];
        onChange(updateValues);
        // Reset dei campi e chiusura dialog
        setNumberOfInfissi(0);
        setSelectedType("");
        setIsOpen(false);
    };

    return (<Dialog open={ isOpen } onOpenChange={ setIsOpen }>
        <DialogTrigger asChild>
            <Button variant="outline" size="icon" type="button">
                <EllipsisIcon />
            </Button>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Aggiungi nuovi infissi</DialogTitle>
                <DialogDescription>Aggiungi nuovi infissi della stessa tipologia</DialogDescription>
            </DialogHeader>
            <div className="flex items-center space-x-2">
                <div className="grid flex-1 gap-2">
                    <Input
                        id="number-of-infissi"
                        placeholder="Inserisci il numero di infissi da aggiungere"
                        value={ numberOfInfissi }
                        onChange={ (e) =>
                            handleInputNumericChange(e, setNumberOfInfissi)
                        }
                    />
                    <Select
                        value={ selectedType }
                        onValueChange={ setSelectedType }
                    >
                        <FormControl>
                            <SelectTrigger className="w-full">
                                <SelectValue placeholder="Seleziona la tipologia" />
                            </SelectTrigger>
                        </FormControl>
                        <SelectContent className="max-h-65">
                            { infissiData.map(infisso => (<SelectItem value={ infisso } key={ infisso }>
                                { infisso }
                            </SelectItem>)) }
                        </SelectContent>
                    </Select>
                </div>
            </div>
            <DialogFooter className="sm:justify-end">
                <Button
                    type="button"
                    variant="default"
                    className="text-white"
                    onClick={ handleAddInfissi }
                >
                    <PlusIcon /><span>Aggiungi</span>
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>);
}
