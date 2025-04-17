import { Card, CardContent, CardHeader }                                 from "@/components/ui/card.tsx";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table.tsx";
import { useEdifici, useStanze }                                         from "@/context/UseProvider.tsx";
import { useMemo }                                                       from "react";

const CardTableStanze = () => {
    const stanzeContext = useStanze();
    const {selectedEdificio} = useEdifici();
    const stanze = useMemo(() => {
        const piani = [
            ...new Set(stanzeContext.data
                                    .filter(value => value.chiave === selectedEdificio)
                                    .map(stanza => stanza.piano))
        ];
        return piani.map(piano => {
            const stanzaPerPiano = stanzeContext.data
                                                .filter(value => value.chiave === selectedEdificio)
                                                .filter(stanza => stanza.piano === piano);
            const stanzeVisitate = stanzaPerPiano.filter(stanza => {
                if (stanza.altezza === undefined) return false; else return stanza.altezza > 0;
            }).length;

            return {
                piano   : piano,
                stanze  : stanzaPerPiano.length,
                visitate: stanzeVisitate
            };
        });

    }, [ selectedEdificio, stanzeContext.data ]);


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
                    { stanze.map(value => {
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