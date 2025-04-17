import { Card, CardContent, CardHeader } from "@/components/ui/card.tsx";
import { Fragment }                      from "react";
import { useEdifici }                    from "@/context/UseProvider.tsx";

const CardDataEdificio = () => {
    const {
              data,
              selectedEdificio
          } = useEdifici();

    return <Card className="@container/card col-span-3">
        <CardHeader>
            <h1 className="text-2xl font-bold text-primary tracking-tight">Dati Edificio</h1>
        </CardHeader>
        <CardContent>
            <div className="grid grid-cols-2 justify-start items-center gap-6">
                { data
                    .filter(value => value.chiave === selectedEdificio)
                    .map((value) => {
                        return Object.entries(value)
                                     .map(([ key, value ]) => {
                                         return <Fragment key={ key }>
                                             <div>
                                                 <p className="font-medium">{ key }</p>
                                             </div>
                                             <div className="flex items-center justify-center">
                                                 <p className="font-semibold">{ value ?? "Dato non disponibile" }</p>
                                             </div>
                                         </Fragment>;
                                     });
                    }) }
            </div>
        </CardContent>
    </Card>;
};

export default CardDataEdificio;