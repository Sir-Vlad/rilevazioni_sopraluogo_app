import TitleCard from "@/components/title-card.tsx";
import {Button} from "@/components/ui/button.tsx";
import {Card, CardContent, CardHeader} from "@/components/ui/card.tsx";
import {Form, FormField, FormItem, FormLabel, FormMessage} from "@/components/ui/form.tsx";
import {Input} from "@/components/ui/input.tsx";
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
import {Switch} from "@/components/ui/switch.tsx";
import {Textarea} from "@/components/ui/textarea.tsx";
import {useSelectedEdificio} from "@/context/SelectedEdificioProvider.tsx";
import {useEdifici} from "@/context/UseProvider.tsx";
import {capitalize, handleInputNumericChange, sanitizeString} from "@/helpers/helpers.ts";
import {IEdificio} from "@/models/models.tsx";
import {zodResolver} from "@hookform/resolvers/zod";
import {CheckIcon, Pencil, PlusIcon, XIcon} from "lucide-react";
import {Fragment, ReactNode} from "react";
import {useForm} from "react-hook-form";
import {z} from "zod";

const FormSchema = z.object({
    anno_costruzione     : z.number().min(1900, {
        message: "Anni di costruzione antecedenti al 1900 inserire il 1900"
    }).max(new Date().getFullYear() - 1, {
        message: "L'anno di costruzione non può essere successivo allo scorso anno"
    }).optional(),
    anno_riqualificazione: z.number().min(1900, {
        message: "Anni di riqualificazione non possono essere precedenti al 1900"
    }).max(new Date().getFullYear(), {
        message: "L'anno di costruzione non può essere successivo al presente anno"
    }).optional(),
    note_riqualificazione: z.string().min(2, {
        message: "Le note devono essere composte da almeno 2 caratteri"
    }).optional(),
    isolamento_tetto     : z.boolean().optional(),
    cappotto             : z.boolean().optional()
});


const CardDataEdificio = () => {
    const {
        data
    } = useEdifici();
    const {edificio} = useSelectedEdificio();

    const valueElement = (value: unknown) => {
        if (typeof value === "boolean") {
            return value ? <CheckIcon className="text-green-500"/> : <XIcon className="text-red-500"/>;
        } else {
            const v: ReactNode = value as ReactNode ?? "Dato non disponibile";
            return <p className="font-semibold">{v}</p>;
        }
    };

    return <Card className="@container/card col-span-3">
        <CardHeader>
            <div className="flex justify-between items-center">
                <TitleCard title="Dati Edificio"/>
                <InsertEdificio/>
            </div>
        </CardHeader>
        <CardContent>
            <div className="grid grid-cols-2 justify-start items-center gap-6">
                {data.length > 0 ? (
                    data
                        .filter(value => value.chiave === edificio?.chiave)
                        .map((value) => {
                            return Object.entries(value)
                                         .filter(([key]) => key !== "note_riqualificazione")
                                         .map(([key, value]) => {
                                             return <Fragment key={key}>
                                    <div>
                                        <p className="font-medium">{capitalize(sanitizeString(key))}</p>
                                    </div>
                                    <div className="flex items-center justify-center">
                                        {valueElement(value)}
                                    </div>
                                </Fragment>;
                                         });

                        })) : (
                    <div className="col-span-2 h-34 flex items-center justify-center rounded-md border p-4">
                    <span>No results</span>
                </div>)}
            </div>
        </CardContent>
    </Card>;
};

const InsertEdificio = () => {
    const {
        modifyEdificio
    } = useEdifici();
    const {edificio} = useSelectedEdificio();


    const form = useForm<z.infer<typeof FormSchema>>({
        resolver     : zodResolver(FormSchema),
        defaultValues: {
            cappotto        : false,
            isolamento_tetto: false
        }
    });

    function onSubmit(dataForm: z.infer<typeof FormSchema>) {
        if (!edificio) {
            return;
        }
        const newEdificio: IEdificio = {
            chiave               : edificio.chiave,
            fascicolo            : edificio.fascicolo,
            indirizzo            : edificio.indirizzo,
            anno_costruzione     : dataForm.anno_costruzione ===
                                   undefined ? undefined : dataForm.anno_costruzione,
            anno_riqualificazione: dataForm.anno_riqualificazione ===
                                   undefined ? undefined : dataForm.anno_riqualificazione,
            note_riqualificazione: dataForm.note_riqualificazione ===
                                   undefined ? undefined : dataForm.note_riqualificazione.toString(),
            isolamento_tetto     : dataForm.isolamento_tetto,
            cappotto             : dataForm.cappotto
        };
        modifyEdificio(newEdificio).catch(console.error);
        form.reset({
            anno_costruzione     : undefined,
            anno_riqualificazione: undefined,
            note_riqualificazione: undefined,
            isolamento_tetto     : false,
            cappotto             : false
        });
    }

    return <Sheet>
        <SheetTrigger asChild>
            <Button variant="ghost" size={"sm"} disabled={edificio === undefined}><PlusIcon/></Button>
        </SheetTrigger>
        <SheetContent className="w-[400px]">
            <SheetHeader>
                <SheetTitle>Aggiungi </SheetTitle>
                <SheetDescription>
                    Compila il form per aggiungere un nuovo tipo a
                </SheetDescription>
            </SheetHeader>
            <Form {...form}>
                <form>
                    <div className="flex flex-col gap-6 px-4">
                        <FormField
                            control={form.control}
                            name={"anno_costruzione"}
                            render={({field}) => {
                                return <FormItem>
                                    <FormLabel>Anno di costruzione</FormLabel>
                                    <Input value={field.value}
                                           onChange={(e) => handleInputNumericChange(e, field.onChange)}
                                    />
                                    <FormMessage/>
                                </FormItem>;
                            }}
                        />
                        <FormField
                            control={form.control}
                            name={"anno_riqualificazione"}
                            render={({field}) => {
                                return <FormItem>
                                    <FormLabel>Anno di riqualificazione</FormLabel>
                                    <Input value={field.value}
                                           onChange={(e) => handleInputNumericChange(e, field.onChange)}
                                    />
                                    <FormMessage/>
                                </FormItem>;
                            }}
                        />
                        <FormField
                            control={form.control}
                            name={"note_riqualificazione"}
                            render={({field}) => {
                                return <FormItem>
                                    <FormLabel>Note sulla riqualificazione</FormLabel>
                                    <Textarea rows={5}
                                              value={field.value}
                                              onChange={field.onChange}
                                              style={{resize: "none"}}
                                    />
                                    <FormMessage/>
                                </FormItem>;
                            }}
                        />
                        <FormField
                            control={form.control}
                            name={"isolamento_tetto"}
                            render={({field}) => {
                                return <FormItem className="flex items-center justify-between gap-2">
                                    <FormLabel>Isolamento del tetto</FormLabel>
                                    <Switch onCheckedChange={field.onChange} checked={field.value}
                                            className="mr-3"/>
                                    <FormMessage/>
                                </FormItem>;
                            }}
                        />
                        <FormField
                            control={form.control}
                            name={"cappotto"}
                            render={({field}) => {
                                return <FormItem className="flex items-center justify-between gap-2">
                                    <FormLabel>Cappotto</FormLabel>
                                    <Switch onCheckedChange={field.onChange} checked={field.value}
                                            className="mr-3"/>
                                    <FormMessage/>
                                </FormItem>;
                            }}
                        />
                    </div>
                </form>
            </Form>
            <SheetFooter className="mt-0">
                <SheetClose asChild>
                    <Button type="button" className="text-white" onClick={async () => {
                        await form.handleSubmit(onSubmit)();
                    }}>
                        <Pencil/>Aggiungi
                    </Button>
                </SheetClose>
            </SheetFooter>
        </SheetContent>
    </Sheet>;
};

export default CardDataEdificio;