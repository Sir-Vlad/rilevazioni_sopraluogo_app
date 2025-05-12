import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select.tsx";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form.tsx";
import { Button } from "@/components/ui/button.tsx";
import { EllipsisIcon, MinusIcon, PlusIcon, Trash } from "lucide-react";
import { useInfissi } from "@/context/UseProvider.tsx";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog.tsx";
import { Input } from "@/components/ui/input.tsx";
import { useState } from "react";
import { handleInputNumericChange } from "@/helpers/helpers.ts";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

interface DynamicSelectProps {
    onChange: (value: string[]) => void;
    values: string[] | undefined;
}

const FormSchema = z.object({
    quantity: z.number().min(1).max(20, {
        message: "Il numero di infissi deve essere minore di 20"
    }),
    type    : z.string().nonempty()
});


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
                                <SelectValue placeholder=""/>
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
                    <PlusIcon/>
                </Button>
                <Button variant="outline" size="icon" type="button"
                        disabled={ values === undefined || values.length === 0 }
                        onClick={ () => {
                            if (!values?.length) return;
                            const newValues = [ ...values ];
                            newValues.pop();
                            onChange(newValues);
                        } }>
                    <MinusIcon/>
                </Button>
                <Button variant="outline" size="icon" type="button"
                        disabled={ values === undefined || values.length === 0 }
                        onClick={ () => {
                            onChange([]);
                        } }>
                    <Trash/>
                </Button>
                <AddInfissiDialog values={ values } onChange={ onChange } infissiData={ infissiData }/>
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
    const form = useForm<z.infer<typeof FormSchema>>({
        resolver     : zodResolver(FormSchema),
        defaultValues: {
            quantity: 0,
            type    : ""
        }
    });

    function onSubmit(data: z.infer<typeof FormSchema>) {
        // Creare un array con il numero specificato di infissi tutti dello stesso tipo
        const newInfissi: string[] = Array(data.quantity).fill(data.type) as string[];
        // Aggiungere questi nuovi infissi all'array esistente
        const updateValues = [ ...(values || []), ...newInfissi ];
        onChange(updateValues);
        setIsOpen(false);
    }

    return (<Dialog open={ isOpen } onOpenChange={ (state) => {
        form.reset();
        setIsOpen(state);
    } }>
        <DialogTrigger asChild>
            <Button variant="outline" size="icon" type="button">
                <EllipsisIcon/>
            </Button>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Aggiungi nuovi infissi</DialogTitle>
                <DialogDescription>Aggiungi nuovi infissi della stessa tipologia</DialogDescription>
            </DialogHeader>
            <Form { ...form }>
                <form onSubmit={ async (e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    return await form.handleSubmit(onSubmit)(e);
                } }>
                    <div className="flex items-center space-x-2">
                        <div className="grid flex-1 gap-2">
                            <FormField
                                control={ form.control }
                                name="quantity"
                                render={ ({ field }) => <FormItem>
                                    <FormLabel>Numero di infissi</FormLabel>
                                    <Input
                                        id="number-of-infissi"
                                        placeholder="Inserisci il numero di infissi da aggiungere"
                                        value={ field.value }
                                        onChange={ (e) => handleInputNumericChange(e, field.onChange) }
                                    />
                                    <FormMessage/>
                                </FormItem> }
                            />
                            <FormField
                                control={ form.control }
                                name={ "type" }
                                render={ ({ field }) => <FormItem>
                                    <FormLabel>Tipologia di infisso</FormLabel>
                                    <Select
                                        value={ field.value }
                                        onValueChange={ field.onChange }
                                    >
                                        <FormControl>
                                            <SelectTrigger className="w-full">
                                                <SelectValue placeholder="Seleziona la tipologia"/>
                                            </SelectTrigger>
                                        </FormControl>
                                        <SelectContent className="max-h-65">
                                            { infissiData.map(infisso => (<SelectItem value={ infisso } key={ infisso }>
                                                { infisso }
                                            </SelectItem>)) }
                                        </SelectContent>
                                    </Select>
                                    <FormMessage/>
                                </FormItem> }/>
                        </div>
                    </div>
                    <div className="flex justify-end pt-4">
                        <Button
                            type="submit"
                            variant="default"
                            className="text-white"
                        >
                            <PlusIcon/><span>Aggiungi</span>
                        </Button>
                    </div>
                </form>
            </Form>
            <DialogFooter className="sm:justify-end">
            </DialogFooter>
        </DialogContent>
    </Dialog>);
}
