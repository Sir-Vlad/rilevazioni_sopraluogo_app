import { Card, CardContent, CardHeader }                                 from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";

const datafake = [
    {
        tipo         : "Calore",
        cod_contatore: "CF45878548",
        indirizzo    : ""
    },
    {
        tipo         : "Elettricità",
        cod_contatore: "1559952285",
        indirizzo    : ""
    },
    {
        tipo         : "Acqua",
        cod_contatore: "78451512",
        indirizzo    : ""
    }
];

const CardUtenzeEdificio = () => {
    // todo: implementare il retrieve nel backend
    return <Card>
        <CardHeader>
            <h1 className="text-2xl font-bold text-primary tracking-tight">Utenze</h1>
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
                        { datafake.length > 0 ?
                            datafake.map(value => {
                                return <TableRow key={ value.cod_contatore }>
                                    <TableCell className="text-center">{ value.tipo }</TableCell>
                                    <TableCell className="text-center">{ value.cod_contatore }</TableCell>
                                    <TableCell className="text-center">{ value.indirizzo }</TableCell>
                                </TableRow>;
                            }) : (
                                <TableCell colSpan={ 5 } className="h-24 text-center">
                                    No results
                                </TableCell>
                            )
                        }
                    </TableBody>
                </Table>
            </div>
        </CardContent>
    </Card>;
};

export default CardUtenzeEdificio;