import CommentsButton from "@/components/annotazione-button.tsx";
import ClearableSelect from "@/components/clearable-select.tsx";
import InputWithMeasureUnit from "@/components/input-with-measure-unit.tsx";
import TitleCard from "@/components/title-card.tsx";
import {Button} from "@/components/ui/button.tsx";
import {Card, CardContent, CardHeader} from "@/components/ui/card";
import {Form, FormField, FormItem, FormLabel, FormMessage} from "@/components/ui/form.tsx";
import {useNotification} from "@/context/NotificationProvider.tsx";
import {useSelectedEdificio} from "@/context/SelectedEdificioProvider.tsx";
import {useDatabase, useInfissi, useTypes} from "@/context/UseProvider.tsx";
import {getSavedFormData, useLocalStorageForm} from "@/hooks/useLocalStorageForm.ts";
import {IAnnotazione, IInfisso} from "@/models/models.tsx";
import {zodResolver} from "@hookform/resolvers/zod";
import {invoke} from "@tauri-apps/api/core";
import {PlusIcon, Trash} from "lucide-react";
import {ChangeEvent, useState} from "react";
import {useForm} from "react-hook-form";
import {toast} from "sonner";
import {z} from "zod";

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

const CardFormInfisso = () => {
    const {
        materialiInfissiType,
        vetroInfissiType,
        tipoInfissi
    } = useTypes();
    const infissi = useInfissi();
    const {
        error
    } = useDatabase();
    const [annotazioni, setAnnotazioni] = useState<string[]>([]);
    const {addNotification} = useNotification();
    const {edificio} = useSelectedEdificio();

    const savedValues = getSavedFormData<typeof FormInfisso>("infissoFormData");

    const form = useForm<z.infer<typeof FormInfisso>>({
        resolver     : zodResolver(FormInfisso),
        defaultValues: savedValues ?? {
            tipo     : tipoInfissi.find(value => value === "FINESTRA") ?? "",
            altezza  : 0,
            larghezza: 0,
            materiale: "",
            vetro    : ""
        }
    });

    useLocalStorageForm(form, "infissoFormData");

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
        if (error === "Database non impostato" || edificio === undefined) {
            toast.warning("Edificio non selezionato");
            return;
        }

        if (data.altezza === 0 && data.larghezza === 0 && data.materiale === "" && data.vetro === "") {
            return;
        }
        const lastInfisso = infissi.data.filter(value => value.id_edificio === edificio.chiave).at(-1);
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
            id         : nextAlphabeticalID(lastInfissoId),
            id_edificio: edificio.chiave
        };
        await infissi.insertInfisso(newInfisso);
        if (annotazioni.length > 0) onSubmitAnnotazioni(newInfisso).then().catch(console.error);
    }

    async function onSubmitAnnotazioni(infisso: IInfisso) {
        for (const content of annotazioni) {
            try {
                const annotazione = {
                    id          : 0,
                    ref_table   : "infisso",
                    id_ref_table: {Infisso: [infisso.id, edificio?.chiave]},
                    content     : content
                } as IAnnotazione;

                await invoke("insert_annotazione", {
                    annotazione: annotazione
                });
            } catch (e) {
                addNotification(e as string, "error");
            }
        }
        setAnnotazioni([]);
        addNotification("Annotazioni inserite con successo", "success");
    }

    function clearForm() {
        form.reset({
            tipo     : tipoInfissi.find(value => value === "FINESTRA") ?? "",
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
    }


    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-1 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6 h-full">
        <Card>
            <CardHeader>
                <div className="flex gap-5 items-center">
                    <TitleCard title="Inserisci Infisso"/>
                    <CommentsButton setAnnotazione={setAnnotazioni} disabled={edificio === undefined}/>
                    <div className="flex flex-1 justify-end">
                        <Button type="button" className="dark:text-white" variant="secondary" onClick={clearForm}>
                            <Trash/> Pulisci Form
                        </Button>
                    </div>
                </div>
            </CardHeader>
            <CardContent>
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)}>
                        <div className="grid grid-cols-12 gap-5">
                            <div className="row-start-1 col-span-12">
                                <div className="grid grid-cols-12 gap-5">
                                    <FormField control={form.control}
                                               name="tipo"
                                               render={({field}) => (
                                                   <div className="col-span-6">
                                                   <FormItem>
                                                       <FormLabel>Tipo</FormLabel>
                                                       <ClearableSelect onChange={field.onChange}
                                                                        value={field.value}
                                                                        options={tipoInfissi}
                                                                        onClear={() => {
                                                                            form.resetField("tipo");
                                                                        }}
                                                       />
                                                   </FormItem>
                                               </div>)}/>
                                </div>
                            </div>
                            <div className="row-start-2 col-span-12">
                                <div className="grid grid-cols-12 gap-5">
                                    <FormField
                                        control={form.control}
                                        name="altezza"
                                        render={({field}) => (
                                            <div className="col-span-6">
                                            <FormItem>
                                                <FormLabel className="flex items-center">
                                                    <p>Altezza</p>
                                                </FormLabel>
                                                <InputWithMeasureUnit
                                                    value={field.value}
                                                    onChange={e => handleInputNumericChange(e, field)}
                                                    disabled={edificio === undefined}
                                                    unitLabel="cm"
                                                />
                                                <FormMessage/>
                                            </FormItem>
                                        </div>)}
                                    />
                                    <FormField
                                        control={form.control}
                                        name="larghezza"
                                        render={({field}) => (
                                            <div className="col-span-6">
                                            <FormItem>
                                                <FormLabel className="flex items-center">
                                                    <p>Larghezza</p>
                                                </FormLabel>
                                                <InputWithMeasureUnit
                                                    value={field.value}
                                                    onChange={e => handleInputNumericChange(e, field)}
                                                    disabled={edificio === undefined}
                                                    unitLabel="cm"
                                                />
                                                <FormMessage/>
                                            </FormItem>
                                        </div>)}
                                    />
                                </div>
                            </div>
                            {/*  Materiale e Vetro  */}
                            <div className="row-start-3 col-span-12">
                                <div className="grid grid-cols-12 gap-5">
                                    <FormField
                                        control={form.control}
                                        name="materiale"
                                        render={({field}) => (
                                            <div className="col-span-6">
                                            <FormItem>
                                                <FormLabel>Materiale</FormLabel>
                                                <ClearableSelect onChange={field.onChange}
                                                                 options={materialiInfissiType} value={field.value}
                                                                 onClear={() => {
                                                                     form.resetField("materiale");
                                                                 }}/>
                                                <FormMessage/>
                                            </FormItem>
                                        </div>)}
                                    />
                                    <FormField
                                        control={form.control}
                                        name="vetro"
                                        render={({field}) => (
                                            <div className="col-span-6">
                                            <FormItem>
                                                <FormLabel>Vetro</FormLabel>
                                                <ClearableSelect onChange={field.onChange}
                                                                 options={vetroInfissiType} value={field.value}
                                                                 onClear={() => {
                                                                     form.resetField("vetro");
                                                                 }}/>
                                                <FormMessage/>
                                            </FormItem>
                                        </div>)}
                                    />
                                </div>
                            </div>
                        </div>
                        <div className="flex items-center justify-end pt-4">
                            <Button type="submit" className="text-white" disabled={edificio === undefined}>
                                <PlusIcon/> <span>Aggiungi Infisso</span>
                            </Button>
                        </div>
                    </form>
                </Form>
            </CardContent>
        </Card>
    </div>;
};

export default CardFormInfisso;