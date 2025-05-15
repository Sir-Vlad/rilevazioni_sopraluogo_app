import { Card, CardContent, CardHeader } from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";
import TitleCard from "@/components/title-card.tsx";
import { useDatabase, useEdifici, useUtenze } from "@/context/UseProvider.tsx";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
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
import { Button } from "@/components/ui/button.tsx";
import { Pencil, PlusIcon } from "lucide-react";
import { Form, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form.tsx";
import { Input } from "@/components/ui/input.tsx";
import ClearableSelect from "@/components/clearable-select.tsx";
import { IUtenza } from "@/models/models.tsx";

const FormSchema = z.object({
    tipo               : z.string(),
    cod_contatore      : z.string(),
    indirizzo_contatore: z.string().optional(),
})


const CardUtenzeEdificio = () => {
    const utenzeContext = useUtenze();

    return <Card>
        <CardHeader>
            <div className="flex justify-between items-center">
                <TitleCard title="Utenze"/>
                <InsertUtenza/>
            </div>
        </CardHeader>
        <CardContent>
            <div className="rounded-md border">
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead className="text-center font-bold">Tipologia</TableHead>
                            <TableHead className="text-center font-bold">Codice</TableHead>
                            <TableHead className="text-center font-bold">Indirizzo</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        { utenzeContext.data.length > 0 ? utenzeContext.data.map(value => {
                            return <TableRow key={ value.cod_contatore }>
                                <TableCell className="text-center">{ value.tipo }</TableCell>
                                <TableCell className="text-center">{ value.cod_contatore }</TableCell>
                                <TableCell className="text-center">{ value.indirizzo_contatore }</TableCell>
                            </TableRow>;
                        }) : (<TableRow>
                            <TableCell colSpan={ 5 } className="h-24 text-center">No results</TableCell>
                        </TableRow>) }
                    </TableBody>
                </Table>
            </div>
        </CardContent>
    </Card>;
};

const InsertUtenza = () => {
    const { selectedEdificio } = useEdifici();
    const { databaseName } = useDatabase();
    const { insertUtenza } = useUtenze();

    const form = useForm<z.infer<typeof FormSchema>>({
        resolver: zodResolver(FormSchema),
    });

    function onSubmit(data: z.infer<typeof FormSchema>) {
        if (!selectedEdificio) {
            return
        }
        if (data.tipo === "elettrica") {
            const codice = data.cod_contatore;
            const regex = /^IT\d{3}E\d{8,9}$/;
            if (codice === "") {
                form.setError("cod_contatore", { message: "Il codice contatore elettrico non può essere vuoto" })
                return;
            }
            if (!regex.test(codice)) {
                form.setError("cod_contatore", { message: "Il codice contatore elettrico non è valido" })
                if (codice.length > 15) {
                    form.setError("cod_contatore", { message: "La lunghezza del codice deve essere 14 o 15 caratteri" })
                }
                return;
            }
        }

        const newUtenza: IUtenza = {
            id                 : 0,
            id_edificio        : selectedEdificio,
            tipo               : data.tipo,
            cod_contatore      : data.cod_contatore,
            indirizzo_contatore: data.indirizzo_contatore
        };
        insertUtenza(newUtenza).catch(console.error);
        form.reset({
            tipo               : "",
            cod_contatore      : "",
            indirizzo_contatore: ""
        });
    }

    return <Sheet>
        <SheetTrigger asChild>
            <Button variant="ghost" size={ "sm" } disabled={ databaseName === null }><PlusIcon/></Button>
        </SheetTrigger>
        <SheetContent className="w-[400px]">
            <SheetHeader>
                <SheetTitle>Aggiungi </SheetTitle>
                <SheetDescription>
                    Compila il form per aggiungere un nuovo tipo a
                </SheetDescription>
            </SheetHeader>
            <Form { ...form }>
                <form>
                    <div className="flex flex-col gap-6 px-4">
                        <FormField
                            control={ form.control }
                            name={ "tipo" }
                            render={ ({ field }) => {
                                return <FormItem>
                                    <FormLabel>Tipo</FormLabel>
                                    <ClearableSelect value={ field.value }
                                                     onChange={ field.onChange }
                                                     options={ [ "Idrica", "Elettrica", "Termica" ] }
                                    />
                                    <FormMessage/>
                                </FormItem>
                            } }
                        />
                        <FormField
                            control={ form.control }
                            name={ "cod_contatore" }
                            render={ ({ field }) => {
                                return <FormItem>
                                    <FormLabel>Codice Contatore</FormLabel>
                                    <Input value={ field.value }
                                           onChange={ field.onChange }
                                    />
                                    <FormMessage/>
                                </FormItem>
                            } }
                        />
                        <FormField
                            control={ form.control }
                            name={ "indirizzo_contatore" }
                            render={ ({ field }) => {
                                return <FormItem>
                                    <FormLabel>Indirizzo Contatore</FormLabel>
                                    <Input value={ field.value }
                                           onChange={ field.onChange }
                                    />
                                    <FormMessage/>
                                </FormItem>
                            } }
                        />
                    </div>
                </form>
            </Form>
            <SheetFooter className="mt-0">
                <SheetClose asChild>
                    <Button type="button" className="text-white" onClick={ async () => {
                        await form.handleSubmit(onSubmit)();
                    } }>
                        <Pencil/>Aggiungi
                    </Button>
                </SheetClose>
            </SheetFooter>
        </SheetContent>
    </Sheet>
}

export default CardUtenzeEdificio;