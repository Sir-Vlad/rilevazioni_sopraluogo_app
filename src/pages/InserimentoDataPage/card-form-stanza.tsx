import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";

import { z } from "zod";
import { ControllerRenderProps, FieldPath, useForm, UseFormReturn } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@/components/ui/button";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import DynamicSelect from "@/components/dynamic-select.tsx";
import { Input } from "@/components/ui/input.tsx";
import { Pencil, PlusIcon, Trash } from "lucide-react";
import CommentButton from "@/components/comment-button.tsx";
import HelpBadge from "@/components/help-badge.tsx";
import { useDatabase, useEdifici, useStanze, useTypes } from "@/context/UseProvider.tsx";
import { handleInputNumericChange } from "@/helpers/helpers";
import { IStanza } from "@/models/models.tsx";
import { toast } from "sonner";
import TitleCard from "@/components/title-card";
import ClearableSelect from "@/components/clearable-select.tsx";
import {
    Sheet,
    SheetClose,
    SheetContent,
    SheetDescription,
    SheetFooter,
    SheetHeader,
    SheetTitle,
    SheetTrigger
} from "@/components/ui/sheet.tsx";
import { Label } from "@/components/ui/label";
import { useState } from "react";


const FormSchema = z.object({
    stanza          : z.string({
        required_error: "Selezionare una stanza"
    }),
    destinazione_uso: z.string(),
    piano           : z.string(),
    altezza         : z.number().max(65000, {
        message: "L'altezza deve essere maggiore di 0 e minore di 65000"
    }).optional(),
    spessore_muro   : z.number().max(256, {
        message: "Lo spessore del muro deve essere maggiore di 0 e minore di 256"
    }).optional(),
    riscaldamento   : z.string().optional(),
    raffrescamento  : z.string().optional(),
    illuminazione   : z.string().optional(),
    infissi         : z.array(z.string()).optional()
});


const CardFormStanza = () => {
    const form = useForm<z.infer<typeof FormSchema>>({
        resolver     : zodResolver(FormSchema),
        defaultValues: {
            altezza      : 0,
            spessore_muro: 0,
            infissi      : []
        }
    });
    const stanzaContext = useStanze();
    const {
        illuminazioneType,
        climatizzazioneType
    } = useTypes();
    const { error } = useDatabase();
    const { selectedEdificio } = useEdifici();

    const stanzeOptions = [ ...[ ...new Set(stanzaContext.data
        .filter(value => value.chiave === selectedEdificio)
        .map((item) => item.stanza)) ]
        .sort((a, b) => {
            if (a.startsWith("_") && !b.startsWith("_")) return -1;
            if (!a.startsWith("_") && b.startsWith("_")) return 1;
            const aNum = Number(a);
            const bNum = Number(b);
            const aIsNum = !isNaN(aNum);
            const bIsNum = !isNaN(bNum);
            if (aIsNum && bIsNum) return aNum - bNum;
            if (aIsNum) return -1;
            if (bIsNum) return 1;
            return a.localeCompare(b);
        }) ];
    const destinazioneUsoOptions = [ ...[ ...new Set(stanzaContext.data.filter(value => value.chiave === selectedEdificio)
        .map((item) => item.destinazione_uso)) ] ];
    const pianoOptions = [ ...[ ...new Set(stanzaContext.data
        .filter(value => value.chiave === selectedEdificio)
        .map((item) => item.piano)) ] ];

    function handleChangeStanza(newValue: string, field: ControllerRenderProps<z.infer<typeof FormSchema>>) {
        field.onChange(newValue);
        const stanzaSelezionata = stanzaContext.data.find((item) => item.stanza === newValue);
        if (stanzaSelezionata) {
            form.setValue("destinazione_uso", stanzaSelezionata.destinazione_uso);
            form.setValue("piano", stanzaSelezionata.piano);
        }
    }

    function onSubmit(data: z.infer<typeof FormSchema>) {
        if (error === "Database non settato") {
            toast.warning("Non hai selezionato un file");
            return;
        }

        const stanze = stanzaContext.data.filter((item) => {
            return item.chiave === selectedEdificio && item.stanza === data.stanza && item.destinazione_uso === data.destinazione_uso && item.piano === data.piano;
        });
        if (stanze.length === 0) {
            toast.error("Stanza non trovata");
            return;
        }
        for (const stanza of stanze) {
            const newStanza: IStanza = {
                ...stanza,
                altezza       : data.altezza,
                spessore_muro : data.spessore_muro,
                riscaldamento : data.riscaldamento,
                raffrescamento: data.raffrescamento,
                illuminazione : data.illuminazione,
                infissi       : data.infissi?.filter(infisso => {
                    return infisso !== null && infisso !== undefined && infisso !== "";
                })
            };
            console.log(newStanza);
            try {
                stanzaContext.updateStanza(newStanza);
                toast.success(`Stanza ${ data.stanza } modificata`);
            } catch (e) {
                toast.error(`Errore durante la modifica della stanza ${ data.stanza }`);
                console.log(e);
            }
        }
    }

    function clearForm() {
        form.reset({
            stanza          : "",
            destinazione_uso: "",
            piano           : "",
            altezza         : 0,
            spessore_muro   : 0,
            riscaldamento   : "",
            raffrescamento  : "",
            illuminazione   : "",
            infissi         : []
        });
    }

    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-1 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6 h-full">
        <Card className="@container/card h-full">
            <CardHeader>
                <CardTitle>
                    <div className="flex gap-5 items-center">
                        <TitleCard title="Modifica Stanza"/>
                        <CommentButton/>
                        <div className="flex flex-1 justify-end">
                            <Button type="button" className="dark:text-white" variant="secondary" onClick={ clearForm }>
                                <Trash/> Pulisci Form
                            </Button>
                        </div>
                    </div>
                </CardTitle>
            </CardHeader>
            <CardContent>
                <Form { ...form }>
                    <form onSubmit={ form.handleSubmit(onSubmit) } className="">
                        <div className="grid grid-cols-12 gap-5">
                            <div className="row-start-1 col-span-12">
                                <div className="grid grid-cols-12 gap-5">
                                    <FormField
                                        control={ form.control }
                                        name="stanza"
                                        render={ ({ field }) => (<div className="col-span-4">
                                            <FormItem>
                                                <FormLabel>Stanza</FormLabel>
                                                <ClearableSelect
                                                    onChange={ (value) => handleChangeStanza(value, field) }
                                                    options={ stanzeOptions }
                                                    value={ field.value }
                                                    onClear={ () => {
                                                        form.reset({
                                                            stanza          : "",
                                                            destinazione_uso: "",
                                                            piano           : ""
                                                        });
                                                    } }
                                                />
                                                <FormMessage/>
                                            </FormItem>
                                        </div>) }
                                    />
                                    <FormField
                                        control={ form.control }
                                        name="destinazione_uso"
                                        render={ ({ field }) => (<div className="col-span-4">
                                            <FormItem>
                                                <FormLabel>Destinazione Uso</FormLabel>
                                                <ClearableSelect onChange={ field.onChange }
                                                                 options={ destinazioneUsoOptions }
                                                                 value={ field.value }/>
                                                <FormMessage/>
                                            </FormItem>
                                        </div>) }
                                    />
                                    <FormField
                                        control={ form.control }
                                        name="piano"
                                        render={ ({ field }) => (<div className="col-span-4">
                                            <FormItem>
                                                <FormLabel>Piano</FormLabel>
                                                <ClearableSelect onChange={ field.onChange }
                                                                 options={ pianoOptions }
                                                                 value={ field.value }/>
                                                <FormMessage/>
                                            </FormItem>
                                        </div>) }
                                    />
                                </div>
                            </div>
                            {/* Altezza e Spessore Muro */ }
                            <div className="row-start-2 col-span-12">
                                <div className="grid grid-cols-12 gap-5">
                                    <FormField
                                        control={ form.control }
                                        name="altezza"
                                        render={ ({ field }) => (<div className="col-span-6">
                                            <FormItem>
                                                <FormLabel className="flex items-center">
                                                    <p>Altezza</p>
                                                    <HelpBadge message="Il valore va inserito in cm"/>
                                                </FormLabel>
                                                <Input value={ field.value }
                                                       onChange={ e => handleInputNumericChange(e, field.onChange) }
                                                />
                                                <FormMessage/>
                                            </FormItem>
                                        </div>) }
                                    />
                                    <FormField
                                        control={ form.control }
                                        name="spessore_muro"
                                        render={ ({ field }) => (<div className="col-span-6">
                                            <FormItem>
                                                <FormLabel className="flex items-center">
                                                    <p>Spessore Muro</p>
                                                    <HelpBadge message="Il valore va inserito in cm"/>
                                                </FormLabel>
                                                <Input value={ field.value }
                                                       onChange={ e => handleInputNumericChange(e, field.onChange) }/>
                                                <FormMessage/>
                                            </FormItem>
                                        </div>) }
                                    />
                                </div>
                            </div>
                            {/* Riscaldamento e Raffrescamento */ }
                            <div className="row-start-3 col-span-12">
                                <div className="grid grid-cols-12 gap-5">
                                    <SelectWithOtherField form={ form } name="riscaldamento"
                                                          label="Riscaldamento"
                                                          options={ climatizzazioneType }/>
                                    <SelectWithOtherField form={ form } name="raffrescamento"
                                                          label="Raffrescamento"
                                                          options={ climatizzazioneType }/>
                                </div>
                            </div>
                            {/* Illuminazione e altro */ }
                            <div className="row-start-4 col-span-6">
                                <SelectWithOtherField form={ form } name="illuminazione"
                                                      label="Illuminazione"
                                                      options={ illuminazioneType }/>
                            </div>
                            {/* Infissi */ }
                            <div className="row-start-6 col-span-12">
                                <FormField
                                    control={ form.control }
                                    name="infissi"
                                    render={ ({ field }) => (<FormItem>
                                        <FormLabel>Infissi</FormLabel>
                                        <FormControl>
                                            <DynamicSelect onChange={ field.onChange } values={ field.value }/>
                                        </FormControl>
                                        <FormMessage/>
                                    </FormItem>) }
                                />
                            </div>
                        </div>
                        <div className="flex justify-end pt-4">
                            <Button type="submit" className="text-white">
                                <Pencil/> <span>Modifica Stanza</span>
                            </Button>
                        </div>
                    </form>
                </Form>
            </CardContent>
        </Card>
    </div>;
};

interface SelectWithOtherFieldProps<TFormValues extends Record<string, unknown>> {
    form: UseFormReturn<TFormValues>;
    name: FieldPath<TFormValues>;
    label: string;
    options: string[];
}


const SelectWithOtherField = <TFormValues extends Record<string, unknown>>({
                                                                               form,
                                                                               name,
                                                                               label,
                                                                               options
                                                                           }: SelectWithOtherFieldProps<TFormValues>) => {
    return (<FormField
        control={ form.control }
        name={ name }
        render={ ({ field }) => (<div className="col-span-6">
            <FormItem>
                <div className="flex justify-between">
                    <FormLabel>{ label }</FormLabel>
                    <SheetAddNewTipo tipo={ label }></SheetAddNewTipo>
                </div>
                <ClearableSelect onChange={ field.onChange } options={ options } value={ field.value as string }
                                 onClear={ () => {
                                     // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                                     // @ts-expect-error
                                     form.reset({
                                         [field.name]: ""
                                     });
                                 } }/>
                <FormMessage/>
            </FormItem>
        </div>) }
    />);
};

const SheetAddNewTipo = ({ tipo }: { tipo: string }) => {
    const [ newTipo, setNewTipo ] = useState("");
    const [ effEnergetica, setEffEnergetica ] = useState(0);

    const handleSubmit = () => {
        console.log(newTipo, effEnergetica);
        toast.warning("Funzionalità non implementata");
        // todo: implementare l'inserimento del nuovo tipo
        setNewTipo("");
        setEffEnergetica(0);
    };


    return <Sheet>
        <SheetTrigger asChild>
            <Button variant="ghost" size={ "sm" }><PlusIcon/></Button>
        </SheetTrigger>
        <SheetContent className="w-[400px]">
            <SheetHeader>
                <SheetTitle>Aggiungi { tipo }</SheetTitle>
                <SheetDescription>
                    Compila il form per aggiungere un nuovo tipo a { tipo.toLowerCase() }
                </SheetDescription>
            </SheetHeader>
            <div className="grid gap-4 p-4">
                <div className="grid grid-cols-4 items-center gap-4">
                    <Label htmlFor="name" className="text-right">
                        Nuovo Tipo
                    </Label>
                    <Input id="name" className="col-span-3" placeholder="Inserisci il nome del nuovo tipo"
                           onChange={ (e) => setNewTipo(e.target.value) }
                           value={ newTipo }
                    />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                    <Label htmlFor="username" className="text-right">
                        Efficienza energetica
                    </Label>
                    <Input id="username" className="col-span-3" placeholder="Inserisci l'efficienza energetica"
                           onChange={ (e) => handleInputNumericChange(e, setEffEnergetica) }
                           value={ effEnergetica }
                    />
                </div>
            </div>
            <SheetFooter className="mt-0">
                <SheetClose asChild>
                    <Button type="button" className="text-white" onClick={ handleSubmit }>
                        <Pencil/>Aggiungi
                    </Button>
                </SheetClose>
            </SheetFooter>
        </SheetContent>
    </Sheet>;
};


export default CardFormStanza;