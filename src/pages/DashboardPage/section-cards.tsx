import { Card, CardDescription, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { useEdifici, useInfissi, useStanze } from "@/context/UseProvider.tsx";

export function SectionCards() {
    const stanzeContext = useStanze();
    const infissiContext = useInfissi();
    const { selectedEdificio } = useEdifici();
    const totPiani = [
        ...new Set(stanzeContext.data
            .filter(value => value.chiave === selectedEdificio)
            .map(value => value.piano))
    ].length;

    const stanzeEdificioSelezionato = stanzeContext.data
        .filter(value => value.chiave === selectedEdificio);
    const totStanze = stanzeEdificioSelezionato.length;
    const infissiList = stanzeEdificioSelezionato.flatMap(value => value.infissi || []);
    const totInfissi = infissiList.length;
    const totMqInfissi = infissiList
        .reduce((totale, infissoId) => {
            const infisso = infissiContext.data.find(i => i.id === infissoId);
            if (infisso) {
                return totale + (infisso.altezza * infisso.larghezza) / 10000;
            }
            return totale;
        }, 0)
        .toFixed(2);


    return (<div
        className="*:data-[slot=card]:shadow-xs @xl/main:grid-cols-2 @5xl/main:grid-cols-4 grid grid-cols-1 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6">
        <Card className="@container/card">
            <CardHeader className="relative text-right">
                <CardDescription className="text-xl">Numero Piani</CardDescription>
                <CardTitle className="@[250px]/card:text-4xl text-2xl font-semibold tabular-nums">
                    { totPiani }
                </CardTitle>
            </CardHeader>
        </Card>
        <Card className="@container/card">
            <CardHeader className="relative text-right">
                <CardDescription className="text-xl">Totale Stanze</CardDescription>
                <CardTitle className="@[250px]/card:text-4xl text-2xl font-semibold tabular-nums">
                    { totStanze }
                </CardTitle>
            </CardHeader>
        </Card>
        <Card className="@container/card">
            <CardHeader className="relative text-right">
                <CardDescription className="text-xl">Totale Infissi</CardDescription>
                <CardTitle className="@[250px]/card:text-4xl text-2xl font-semibold tabular-nums">
                    { totInfissi }
                </CardTitle>
            </CardHeader>
        </Card>
        <Card className="@container/card">
            <CardHeader className="relative text-right">
                <CardDescription className="text-xl">Mq Infissi</CardDescription>
                <CardTitle className="@[250px]/card:text-4xl text-2xl font-semibold tabular-nums">
                    { totMqInfissi } <span className="text-2xl">m<sup>2</sup></span>
                </CardTitle>
            </CardHeader>
        </Card>
    </div>);
}
