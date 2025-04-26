import { Card, CardContent, CardHeader }                                 from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";
import TitleCard                                                         from "@/components/title-card.tsx";

const datafake = [
    {
        id          : 1,
        potenza     : 85,
        proprietario: "Mario"
    },
    {
        id          : 2,
        potenza     : 85,
        proprietario: "Ugo"
    }
];


const CardFotovoltaico = () => {
    // todo: implementare il retrieve nel backend
    return <Card>
        <CardHeader>
            <TitleCard title="Fotovoltaico" />
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
                        { datafake.length > 0 ?
                            (datafake.map(value => {
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