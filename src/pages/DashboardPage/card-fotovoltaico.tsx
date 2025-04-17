import { Card, CardContent, CardHeader }                                 from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";

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
            <h1 className="text-2xl font-bold text-primary tracking-tight">Fotovoltaico</h1>
        </CardHeader>
        <CardContent>
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead className="text-center font-bold">Numero</TableHead>
                        <TableHead className="text-center font-bold">Potenza</TableHead>
                        <TableHead className="text-center font-bold">Proprietario</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {
                        datafake.map(value => {
                            return <TableRow key={ value.id }>
                                <TableCell className="text-center">{ value.id }</TableCell>
                                <TableCell className="text-center">{ value.potenza } KW</TableCell>
                                <TableCell className="text-center">{ value.proprietario }</TableCell>
                            </TableRow>;
                        })
                    }
                </TableBody>
            </Table>
        </CardContent>
    </Card>;
};

export default CardFotovoltaico;