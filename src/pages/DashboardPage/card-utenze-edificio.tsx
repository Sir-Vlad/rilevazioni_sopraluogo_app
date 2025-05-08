import { Card, CardContent, CardHeader }                                 from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";
import TitleCard                                                         from "@/components/title-card.tsx";
import { useUtenze }                                                     from "@/context/UseProvider.tsx";

const CardUtenzeEdificio = () => {
    const utenzeContext = useUtenze();

    return <Card>
        <CardHeader>
            <TitleCard title="Utenze" />
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
                        { utenzeContext.data.length > 0 ?
                            utenzeContext.data.map(value => {
                                return <TableRow key={ value.cod_contatore }>
                                    <TableCell className="text-center">{ value.tipo }</TableCell>
                                    <TableCell className="text-center">{ value.cod_contatore }</TableCell>
                                    <TableCell className="text-center">{ value.indirizzo_contatore }</TableCell>
                                </TableRow>;
                            }) : (
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

export default CardUtenzeEdificio;