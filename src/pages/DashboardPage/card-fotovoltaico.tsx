import InputWithMeasureUnit from "@/components/input-with-measure-unit.tsx";
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
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table.tsx";
import {useSelectedEdificio} from "@/context/SelectedEdificioProvider.tsx";
import {useFotovoltaico} from "@/context/UseProvider.tsx";
import {IFotovoltaico} from "@/models/models.tsx";
import {zodResolver} from "@hookform/resolvers/zod";
import {Pencil, PlusIcon} from "lucide-react";
import {ChangeEvent} from "react";
import {ControllerRenderProps, useForm} from "react-hook-form";
import {z} from "zod";

const FormSchema = z.object({
    potenza     : z.number().gt(0.0, {
        message: "La potenza deve essere maggiore di 0"
    }),
    proprietario: z.string().min(2, {
        message: "Il nome deve essere composto da almeno 2 caratteri"
    })
});


const CardFotovoltaico = () => {
    const fotovoltaicoContext = useFotovoltaico();

    return <Card>
        <CardHeader>
            <div className="flex justify-between items-center">
                <TitleCard title="Fotovoltaico"/>
                <InsertFotovoltaico/>
            </div>
        </CardHeader>
        <CardContent>
            <div className="rounded-md border">
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead className="text-center font-bold">Numero</TableHead>
                            <TableHead className="text-center font-bold">Potenza</TableHead>
                            <TableHead className="text-center font-bold">Proprietario</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {fotovoltaicoContext.data.length > 0 ? (
                            fotovoltaicoContext.data.map(value => {
                                return <TableRow key={value.id}>
                                <TableCell className="text-center">{value.id}</TableCell>
                                <TableCell className="text-center">{value.potenza} KW</TableCell>
                                <TableCell className="text-center">{value.proprietario}</TableCell>
                            </TableRow>;
                            })) : (
                            <TableRow>
                            <TableCell colSpan={5} className="h-24 text-center">No results</TableCell>
                        </TableRow>)}
                    </TableBody>
                </Table>
            </div>
        </CardContent>
    </Card>;
};

const InsertFotovoltaico = () => {
    const {insertFotovoltaico} = useFotovoltaico();
    const {edificio} = useSelectedEdificio();


    const form = useForm<z.infer<typeof FormSchema>>({
        resolver     : zodResolver(FormSchema),
        defaultValues: {
            potenza     : 0.0,
            proprietario: ""
        }
    });

    function onSubmit(data: z.infer<typeof FormSchema>) {
        if (!edificio) {
            return;
        }

        const newFotovoltaico: IFotovoltaico = {
            id          : 0,
            id_edificio : edificio?.chiave,
            potenza     : data.potenza,
            proprietario: data.proprietario
        };
        insertFotovoltaico(newFotovoltaico).catch(console.error);
        form.reset();
    }

    function onChangeInput(e: ChangeEvent<HTMLInputElement>, field: ControllerRenderProps<z.infer<typeof FormSchema>>) {
        const {value} = e.target;
        if (value === "") {
            field.onChange(0.0);
            return;
        }

        const regex = /^\d+(\.\d{1,2})?$/;
        if (regex.test(value)) {
            field.onChange(parseFloat(value));
        } else {
            const split = value.replace(/[^0-9.]/g, "").split(".");
            if (split.length >= 2) {
                const decimal = split[1].substring(0, 2);
                const newValue = split[0] + "." + decimal;
                field.onChange(decimal === "" ? newValue : parseFloat(newValue));
            }
        }
    }


    return <Sheet>
        <SheetTrigger asChild>
            <Button variant="ghost" size={"sm"} disabled={edificio?.chiave === null}><PlusIcon/></Button>
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
                            name={"potenza"}
                            render={({field}) => {
                                return <FormItem>
                                    <FormLabel>Potenza</FormLabel>
                                    <InputWithMeasureUnit
                                        unitLabel="KWatt"
                                        value={field.value}
                                        onChange={(e) => onChangeInput(e, field)}/>
                                    <FormMessage/>
                                </FormItem>;
                            }}
                        />
                        <FormField
                            control={form.control}
                            name={"proprietario"}
                            render={({field}) => {
                                return <FormItem>
                                    <FormLabel>Proprietario</FormLabel>
                                    <Input value={field.value}
                                           onChange={field.onChange}
                                    />
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


export default CardFotovoltaico;