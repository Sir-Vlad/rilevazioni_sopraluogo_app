import TitleCard from "@/components/title-card";
import {Card, CardContent, CardHeader} from "@/components/ui/card.tsx";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table.tsx";
import {useSelectedEdificio} from "@/context/SelectedEdificioProvider.tsx";
import {useStanze} from "@/context/UseProvider.tsx";
import {useMemo} from "react";

const CardTableStanze = () => {
    const stanzeContext = useStanze();
    const {edificio} = useSelectedEdificio();

    const stanze = useMemo(() => {
        const piani = [
            ...new Set(stanzeContext.data
                                    .filter(value => value.edificio_id === edificio?.chiave)
                                    .map(stanza => stanza.piano))
        ];
        return piani.map(piano => {
            const stanzaPerPiano = stanzeContext.data
                                                .filter(value => value.edificio_id === edificio?.chiave)
                                                .filter(stanza => stanza.piano === piano);
            const stanzeVisitate = stanzaPerPiano.filter(stanza => {
                if (stanza.altezza === undefined) return false; else return stanza.altezza >= 0;
            }).length;

            return {
                piano   : piano,
                stanze  : stanzaPerPiano.length,
                visitate: stanzeVisitate
            };
        });

    }, [edificio?.chiave, stanzeContext.data]);


    return <Card className="@container/card col-span-5">
        <CardHeader>
            <TitleCard title="Stanze"/>
        </CardHeader>
        <CardContent>
            <div className="rounded-md border">
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
                        {stanze.length > 0 ?
                            (
                                stanze.map(value => {
                                    const visitPercentage = Math.ceil(value.visitate / value.stanze * 100);
                                    return <TableRow key={value.piano}>
                                    <TableCell className="text-center">{value.piano}</TableCell>
                                    <TableCell className="text-center">{value.stanze}</TableCell>
                                    <TableCell className="text-center">{value.visitate}</TableCell>
                                    <TableCell className="text-center">{visitPercentage} %</TableCell>
                                </TableRow>;
                                })) : (
                                <TableRow>
                                    <TableCell colSpan={5} className="h-24 text-center">No results</TableCell>
                                </TableRow>
                            )}
                    </TableBody>
                </Table>
            </div>
        </CardContent>
    </Card>;
};

export default CardTableStanze;