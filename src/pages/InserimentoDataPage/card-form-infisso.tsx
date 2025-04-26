import { Card, CardContent, CardHeader }                                  from "@/components/ui/card";
import { z }                                                              from "zod";
import { useForm }                                                        from "react-hook-form";
import { zodResolver }                                                    from "@hookform/resolvers/zod";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue }  from "@/components/ui/select.tsx";
import { Input }                                                          from "@/components/ui/input.tsx";
import { ChangeEvent, Fragment }                                          from "react";
import { Button }                                                         from "@/components/ui/button.tsx";
import { PlusIcon }                                                       from "lucide-react";
import { useDatabase, useInfissi, useTypes }                              from "@/context/UseProvider.tsx";
import CommentsButton                                                     from "@/components/comment-button.tsx";
import { toast }                                                          from "sonner";
import { IInfisso }                                                       from "@/models/models.tsx";
import HelpBadge                                                          from "@/components/help-badge.tsx";
import TitleCard                                                          from "@/components/title-card.tsx";

const nextAlphabeticalID = (prevID: string | null) => {
    if (!prevID || prevID === "") return "A";
    let result = "";
    let carry = true;
    for (let i = prevID.length - 1; i >= 0; i--) {
        const char = prevID[i];
        if (carry) {
            if (char === "Z") {
                result = "A" + result;
            } else {
                result = String.fromCharCode(char.charCodeAt(0) + 1) + result;
                carry = false;
            }
        } else {
            result = char + result;
        }
    }
    return carry ? "A" + result : result;
};

const FormInfisso = z.object({
    tipo     : z.string(),
    altezza  : z.number().positive().max(65000, {
        message: "L'altezza deve essere maggiore di 0 e minore di 65000"
    }),
    larghezza: z.number().positive().max(65000, {
        message: "La larghezza deve essere maggiore di 0 e minore di 65000"
    }),
    materiale: z.string(),
    vetro    : z.string()
});

const infissiType = [ "FINESTRA", "PORTA" ];

const CardFormInfisso = () => {
    const form = useForm<z.infer<typeof FormInfisso>>({
        resolver     : zodResolver(FormInfisso),
        defaultValues: {
            tipo     : infissiType[0],
            altezza  : 0,
            larghezza: 0
        }
    });
    const {
              materialiInfissiType,
              vetroInfissiType
          } = useTypes();
    const infissi = useInfissi();
    const {error} = useDatabase();


    const handleInputNumericChange = (event: ChangeEvent<HTMLInputElement>, field: {
        onChange: (value: number) => void
    }) => {
        const {value} = event.target;
        if (value.length === 0) {
            field.onChange(0);
            return;
        }
        if (/^\D$/.test(value[value.length - 1])) {
            const newValue = value.slice(0, -1);
            if (newValue.length === 0) {
                field.onChange(0);
            } else {
                field.onChange(Number(newValue));
            }
            return;
        }
        field.onChange(Number(value));
    };

    async function onSubmit(data: z.infer<typeof FormInfisso>) {
        if (error === "Database non impostato") {
            toast.warning("File non selezionato");
            return;
        }

        if (data.altezza === 0 && data.larghezza === 0 && data.materiale === "" && data.vetro === "") {
            return;
        }
        const lastInfisso = infissi.data.at(-1);
        let lastInfissoId = "";
        if (lastInfisso) {
            if (lastInfisso.id) {
                lastInfissoId = lastInfisso.id;
            } else {
                throw new Error("Infisso non ha un id");
            }
        }
        const newInfisso: IInfisso = {
            ...data,
            id: nextAlphabeticalID(lastInfissoId)
        };
        try {
            await infissi.insertInfisso(newInfisso);
            toast.success("Infisso inserito con successo");
            form.reset({
                tipo     : infissiType[0],
                altezza  : 0,
                larghezza: 0,
                materiale: "",
                vetro    : ""
            }, {
                keepErrors     : false,
                keepDirty      : false,
                keepIsSubmitted: false,
                keepTouched    : false,
                keepIsValid    : false,
                keepSubmitCount: false

            });
        } catch (e) {
            toast.error("Errore durante l'inserimento del nuovo infisso");
            console.error(e);
        }
    }


    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-1 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6 h-full">
        <Card>
            <CardHeader>
                <div className="flex gap-5 items-center">
                    <TitleCard title="Inserisci Infisso" />
                    <CommentsButton />
                </div>
            </CardHeader>
            <CardContent>
                <Form { ...form }>
                    <form onSubmit={ form.handleSubmit(onSubmit) }>
                        <div className="grid grid-cols-12 gap-5">
                            <div className="row-start-1 col-span-12">
                                <div className="grid grid-cols-12 gap-5">
                                    <FormField control={ form.control }
                                               name="tipo"
                                               render={ ({field}) => (<div className="col-span-6">
                                                   <FormItem>
                                                       <FormLabel>Tipo</FormLabel>
                                                       <Select onValueChange={ field.onChange }
                                                               value={ field.value }>
                                                           <FormControl>
                                                               <SelectTrigger className="w-full">
                                                                   <SelectValue
                                                                       placeholder="Seleziona un tipo di infisso" />
                                                               </SelectTrigger>
                                                           </FormControl>
                                                           <SelectContent>
                                                               { infissiType.map((value, index) => (
                                                                   <Fragment key={ index + 1 }>
                                                                       <SelectItem
                                                                           value={ value }>{ value }</SelectItem>
                                                                   </Fragment>)) }
                                                           </SelectContent>
                                                       </Select>
                                                   </FormItem>
                                               </div>) } />
                                </div>
                            </div>
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
                                                       onChange={ e => handleInputNumericChange(e, field) }
                                                />
                                                <FormMessage />
                                            </FormItem>
                                        </div>) }
                                    />
                                    <FormField
                                        control={ form.control }
                                        name="larghezza"
                                        render={ ({field}) => (<div className="col-span-6">
                                            <FormItem>
                                                <FormLabel className="flex items-center">
                                                    <p>Larghezza</p>
                                                    <HelpBadge message="Il valore va inserito in cm" />
                                                </FormLabel>
                                                <Input value={ field.value }
                                                       onChange={ e => handleInputNumericChange(e, field) }
                                                />
                                                <FormMessage />
                                            </FormItem>
                                        </div>) }
                                    />
                                </div>
                            </div>
                            {/*  Materiale e Vetro  */ }
                            <div className="row-start-3 col-span-12">
                                <div className="grid grid-cols-12 gap-5">
                                    <FormField
                                        control={ form.control }
                                        name="materiale"
                                        render={ ({field}) => (<div className="col-span-6">
                                            <FormItem>
                                                <FormLabel>Materiale</FormLabel>
                                                <Select onValueChange={ field.onChange }
                                                        defaultValue={ field.value } value={ field.value }>
                                                    <FormControl>
                                                        <SelectTrigger className="w-full">
                                                            <SelectValue placeholder="Seleziona un tipo di materiale" />
                                                        </SelectTrigger>
                                                    </FormControl>
                                                    <SelectContent>
                                                        { materialiInfissiType.map((value, index) => (
                                                            <Fragment key={ index + 1 }>
                                                                <SelectItem value={ value }>{ value }</SelectItem>
                                                            </Fragment>)) }
                                                    </SelectContent>
                                                </Select>
                                                <FormMessage />
                                            </FormItem>
                                        </div>) }
                                    />
                                    <FormField
                                        control={ form.control }
                                        name="vetro"
                                        render={ ({field}) => (<div className="col-span-6">
                                            <FormItem>
                                                <FormLabel>Vetro</FormLabel>
                                                <Select onValueChange={ field.onChange }
                                                        defaultValue={ field.value } value={ field.value }>
                                                    <FormControl>
                                                        <SelectTrigger className="w-full">
                                                            <SelectValue placeholder="Seleziona un tipo di vetro" />
                                                        </SelectTrigger>
                                                    </FormControl>
                                                    <SelectContent>
                                                        { vetroInfissiType.map((value, index) => (
                                                            <Fragment key={ index + 1 }>
                                                                <SelectItem value={ value }>{ value }</SelectItem>
                                                            </Fragment>)) }
                                                    </SelectContent>
                                                </Select>
                                                <FormMessage />
                                            </FormItem>
                                        </div>) }
                                    />
                                </div>
                            </div>
                        </div>
                        <div className="flex justify-end pt-4">
                            <Button type="submit" className="text-white">
                                <PlusIcon /> <span>Aggiungi Infisso</span>
                            </Button>
                        </div>
                    </form>
                </Form>
            </CardContent>
        </Card>
    </div>;
};

export default CardFormInfisso;