import { Card, CardContent, CardHeader } from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";
import TitleCard from "@/components/title-card.tsx";
import { useFotovoltaico } from "@/context/UseProvider.tsx";

const CardFotovoltaico = () => {
    const fotovoltaicoContext = useFotovoltaico();

    return <Card>
        <CardHeader>
            <TitleCard title="Fotovoltaico"/>
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
                        { fotovoltaicoContext.data.length > 0 ?
                            (fotovoltaicoContext.data.map(value => {
                                return <TableRow key={ value.id }>
                                    <TableCell className="text-center">{ value.id }</TableCell>
                                    <TableCell className="text-center">{ value.potenza } KW</TableCell>
                                    <TableCell className="text-center">{ value.proprietario }</TableCell>
                                </TableRow>;
                            })) : (
                                <TableRow>
                                    <TableCell colSpan={ 5 } className="h-24 text-center">No results</TableCell>
                                </TableRow>
                            )
                        }
                    </TableBody>
                </Table>
            </div>
        </CardContent>
    </Card>;
};

export default CardFotovoltaico;