import { Card, CardContent, CardHeader }                                 from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";

const datafake = [
    {
        piano   : -1,
        stanze  : 15,
        visitate: 0
    }, {
        piano   : "T",
        stanze  : 40,
        visitate: 15
    }, {
        piano   : 1,
        stanze  : 40,
        visitate: 0
    }, {
        piano   : 2,
        stanze  : 12,
        visitate: 0
    }
];

const CardTableStanze = () => {
    return <Card className="@container/card col-span-5">
        <CardHeader>
            <h1 className="text-2xl font-bold text-primary tracking-tight">Stanze</h1>
        </CardHeader>
        <CardContent>
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead className="text-center font-bold">Piani</TableHead>
                        <TableHead className="text-center font-bold">Numero di stanze</TableHead>
                        <TableHead className="text-center font-bold">Stanze visitate</TableHead>
                        <TableHead className="text-center font-bold">Completamento</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    { datafake.map(value => {
                        const visitPercentage = Math.ceil(value.visitate / value.stanze * 100);
                        return <TableRow key={ value.piano }>
                            <TableCell className="text-center">{ value.piano }</TableCell>
                            <TableCell className="text-center">{ value.stanze }</TableCell>
                            <TableCell className="text-center">{ value.visitate }</TableCell>
                            <TableCell className="text-center">{ visitPercentage } %</TableCell>
                        </TableRow>;
                    }) }
                </TableBody>
            </Table>
        </CardContent>
    </Card>;
};

export default CardTableStanze;