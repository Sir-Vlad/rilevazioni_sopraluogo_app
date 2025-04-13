import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";

import { z }                                                              from "zod";
import { ControllerRenderProps, FieldPath, useForm, UseFormReturn }       from "react-hook-form";
import { zodResolver }                                                    from "@hookform/resolvers/zod";
import { Button }                                                         from "@/components/ui/button";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue }  from "@/components/ui/select";
import DynamicSelect                                                      from "@/components/dynamic-select.tsx";
import { Input }                                                          from "@/components/ui/input.tsx";
import { Fragment }                                                       from "react";
import { Pencil }                                                         from "lucide-react";
import CommentButton                                                      from "@/components/comment-button.tsx";
import HelpBadge                                                          from "@/components/help-badge.tsx";
import { useDatabase, useStanze, useTypes }                               from "@/context/UseProvider.tsx";
import { handleInputNumericChange }                                       from "@/helpers/helpers";
import { IStanza }                                                        from "@/models/models.tsx";
import { toast }                                                          from "sonner";


const FormSchema = z.object({
    stanza              : z.string({
        required_error: "Selezionare una stanza"
    }),
    destinazione_uso    : z.string(),
    piano               : z.string(),
    altezza             : z.number().min(0).optional(),
    spessore_muro       : z.number().min(0).optional(),
    riscaldamento       : z.string().optional(),
    riscaldamento_altro : z.string().optional(),
    raffrescamento      : z.string().optional(),
    raffrescamento_altro: z.string().optional(),
    illuminazione       : z.string().optional(),
    illuminazione_altro : z.string().optional(),
    infissi             : z.array(z.string()).optional()
});


const CardFormStanza = () => {
    const form = useForm<z.infer<typeof FormSchema>>({
        resolver     : zodResolver(FormSchema),
        defaultValues: {
            altezza      : 0,
            spessore_muro: 0
        }
    });
    const stanzaContext = useStanze();
    const {
              illuminazioneType,
              climatizzazioneType
          } = useTypes();
    const {error} = useDatabase();

    const stanzeOptions = [
        ...[ ...new Set(stanzaContext.data.map((item) => item.stanza)) ]
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
            })
    ];
    const destinazioneUsoOptions = [
        ...[ ...new Set(stanzaContext.data.map((item) => item.destinazione_uso)) ]
    ];
    const pianoOptions = [
        ...[ ...new Set(stanzaContext.data.map((item) => item.piano)) ]
    ];

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

        const stanza = stanzaContext.data.find((item) => {
            return item.stanza === data.stanza && item.destinazione_uso === data.destinazione_uso && item.piano === data.piano;
        });
        if (stanza === undefined) {
            toast.error("Stanza non trovata");
            return;
        }
        const newStanza: IStanza = {
            ...stanza,
            altezza       : data.altezza,
            spessore_muro : data.spessore_muro,
            riscaldamento : data.riscaldamento === "Altro" ? data.riscaldamento_altro : data.riscaldamento,
            raffrescamento: data.raffrescamento === "Altro" ? data.raffrescamento_altro : data.raffrescamento,
            illuminazione : data.illuminazione === "Altro" ? data.illuminazione_altro : data.illuminazione,
            infissi       : data.infissi
        };
        try {
            stanzaContext.updateStanza(newStanza);
            toast.success(`Stanza ${ data.stanza } modificata`);
        } catch (e) {
            toast.error("Errore durante la modifica della stanza");
            console.log(e);
        }
    }

    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-1 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6 h-full">
        <Card className="@container/card h-full">
            <CardHeader>
                <CardTitle>
                    <div className="flex gap-5 items-center">
                        <h1 className="text-xl font-bold text-primary tracking-tight">Modifica Stanza</h1>
                        <CommentButton />
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
                                        render={ ({field}) => (<div className="col-span-4">
                                            <FormItem>
                                                <FormLabel>Stanza</FormLabel>
                                                <Select onValueChange={ (value) => handleChangeStanza(value, field) }
                                                        defaultValue={ field.value } value={ field.value }>
                                                    <FormControl>
                                                        <SelectTrigger className="w-full">
                                                            <SelectValue
                                                                placeholder="Seleziona una stanza" />
                                                        </SelectTrigger>
                                                    </FormControl>
                                                    <SelectContent>
                                                        { stanzeOptions.map(value => {
                                                            return <Fragment key={ value }>
                                                                <SelectItem
                                                                    value={ value }
                                                                >{ value }</SelectItem>
                                                            </Fragment>;
                                                        }) }
                                                    </SelectContent>
                                                </Select>
                                                <FormMessage />
                                            </FormItem>
                                        </div>) }
                                    />
                                    <FormField
                                        control={ form.control }
                                        name="destinazione_uso"
                                        render={ ({field}) => (<div className="col-span-4">
                                            <FormItem>
                                                <FormLabel>Destinazione Uso</FormLabel>
                                                <Select onValueChange={ field.onChange }
                                                        defaultValue={ field.value } value={ field.value }>
                                                    <FormControl>
                                                        <SelectTrigger className="w-full">
                                                            <SelectValue placeholder="" />
                                                        </SelectTrigger>
                                                    </FormControl>
                                                    <SelectContent>
                                                        { destinazioneUsoOptions.map(value => {
                                                            return <Fragment key={ value }>
                                                                <SelectItem value={ value }>{ value }</SelectItem>
                                                            </Fragment>;
                                                        }) }
                                                    </SelectContent>
                                                </Select>
                                                <FormMessage />
                                            </FormItem>
                                        </div>) }
                                    />
                                    <FormField
                                        control={ form.control }
                                        name="piano"
                                        render={ ({field}) => (<div className="col-span-4">
                                            <FormItem>
                                                <FormLabel>Piano</FormLabel>
                                                <Select onValueChange={ field.onChange }
                                                        defaultValue={ field.value } value={ field.value }>
                                                    <FormControl>
                                                        <SelectTrigger className="w-full">
                                                            <SelectValue
                                                                placeholder="" />
                                                        </SelectTrigger>
                                                    </FormControl>
                                                    <SelectContent>
                                                        { pianoOptions.map(value => {
                                                            return <Fragment key={ value }>
                                                                <SelectItem value={ value }>{ value }</SelectItem>
                                                            </Fragment>;
                                                        }) }
                                                    </SelectContent>
                                                </Select>
                                                <FormMessage />
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
                                        render={ ({field}) => (<div className="col-span-6">
                                            <FormItem>
                                                <FormLabel className="flex items-center">
                                                    <p>Altezza</p>
                                                    <HelpBadge message="Il valore va inserito in cm" />
                                                </FormLabel>
                                                <Input value={ field.value }
                                                       onChange={ e => handleInputNumericChange(e, field.onChange) }
                                                />
                                                <FormMessage />
                                            </FormItem>
                                        </div>) }
                                    />
                                    <FormField
                                        control={ form.control }
                                        name="spessore_muro"
                                        render={ ({field}) => (<div className="col-span-6">
                                            <FormItem>
                                                <FormLabel className="flex items-center">
                                                    <p>Spessore Muro</p>
                                                    <HelpBadge message="Il valore va inserito in cm" />
                                                </FormLabel>
                                                <Input value={ field.value }
                                                       onChange={ e => handleInputNumericChange(e, field.onChange) } />
                                                <FormMessage />
                                            </FormItem>
                                        </div>) }
                                    />
                                </div>
                            </div>
                            {/* Riscaldamento e altro */ }
                            <div className="row-start-3 col-span-12">
                                <SelectWithOtherField form={ form } name="riscaldamento"
                                                      label="Riscaldamento" otherLabel="Specifica altro"
                                                      options={ climatizzazioneType } />
                            </div>
                            {/* Raffrescamento e altro */ }
                            <div className="row-start-4 col-span-12">
                                <SelectWithOtherField form={ form } name="raffrescamento"
                                                      label="Raffrescamento" otherLabel="Specifica altro"
                                                      options={ climatizzazioneType } />
                            </div>
                            {/* Illuminazione e altro */ }
                            <div className="row-start-5 col-span-12">
                                <SelectWithOtherField form={ form } name="illuminazione"
                                                      label="Illuminazione" otherLabel="Specifica altro"
                                                      options={ illuminazioneType } />
                            </div>
                            {/* Infissi */ }
                            <div className="row-start-6 col-span-12">
                                <FormField
                                    control={ form.control }
                                    name="infissi"
                                    render={ ({field}) => (<FormItem>
                                        <FormLabel>Infissi</FormLabel>
                                        <FormControl>
                                            <DynamicSelect onChange={ field.onChange } values={ field.value } />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>) }
                                />
                            </div>
                        </div>
                        <div className="flex justify-end pt-4">
                            <Button type="submit" className="text-white">
                                <Pencil /> <span>Modifica Stanza</span>
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
    otherLabel: string;
    options: string[];
}


const SelectWithOtherField = <TFormValues extends Record<string, unknown>>({
                                                                               form,
                                                                               name,
                                                                               label,
                                                                               otherLabel,
                                                                               options
                                                                           }: SelectWithOtherFieldProps<TFormValues>) => {
    return (<div className="grid grid-cols-12 gap-5">
        <FormField
            control={ form.control }
            name={ name }
            render={ ({field}) => (<div className="col-span-6">
                <FormItem>
                    <FormLabel>{ label }</FormLabel>
                    <Select onValueChange={ field.onChange } defaultValue={ field.value as string }>
                        <FormControl>
                            <SelectTrigger className="w-full">
                                <SelectValue placeholder={ `Seleziona ${ label.toLowerCase() }` } />
                            </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                            { options.map((value, index) => (<Fragment key={ index + 1 }>
                                <SelectItem value={ value }>{ value }</SelectItem>
                            </Fragment>)) }
                            <SelectItem value="Altro">Altro</SelectItem>
                        </SelectContent>
                    </Select>
                    <FormMessage />
                </FormItem>
            </div>) }
        />
        <FormField
            control={ form.control }
            name={ `${ name }_altro` as FieldPath<TFormValues> }
            render={ ({field}) => (<div className="col-span-6">
                <FormItem>
                    <FormLabel>{ otherLabel }</FormLabel>
                    <Input
                        onChange={ field.onChange }
                        disabled={ form.watch(name) !== "Altro" }
                    />
                    <FormMessage />
                </FormItem>
            </div>) }
        />
    </div>);
};


export default CardFormStanza;