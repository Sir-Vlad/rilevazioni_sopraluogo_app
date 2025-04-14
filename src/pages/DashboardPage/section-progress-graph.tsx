import CardProgress   from "@/pages/DashboardPage/card-progress.tsx";
import { useInfissi } from "@/context/UseProvider.tsx";
import { useMemo }    from "react";

const SectionProgressGraph = () => {
    const infissiContext = useInfissi();

    const materialiValues = useMemo(
        () => {
            const conteggioMateriali: Map<string, number> = new Map();
            infissiContext.data.forEach(infisso => {
                const materiali = infisso.materiale;
                conteggioMateriali.set(materiali, (conteggioMateriali.get(materiali) ?? 0) + 1);
            });
            return Array.from(conteggioMateriali.entries()).map(([ key, value ]) => ({
                label: key,
                value: value
            }));
        },
        [ infissiContext.data ]
    );

    const vetroValues = useMemo(
        () => {
            const conteggioMateriali: Map<string, number> = new Map();
            infissiContext.data.forEach(infisso => {
                const vetro = infisso.vetro;
                conteggioMateriali.set(vetro, (conteggioMateriali.get(vetro) ?? 0) + 1);
            });
            return Array.from(conteggioMateriali.entries()).map(([ key, value ]) => ({
                label: key,
                value: value
            }));
        },
        [ infissiContext.data ]
    );


    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-2 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6">
        <CardProgress title={ "Materiali" } values={ materialiValues } />
        <CardProgress title={ "Vetro" } values={ vetroValues } />
    </div>;
};

export default SectionProgressGraph;