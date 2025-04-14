import { Card, CardContent, CardHeader } from "@/components/ui/card.tsx";
import { Fragment }                      from "react";

const dataFake = [
    {
        label: "Fascicolo",
        value: 100
    }, {
        label: "Chiave",
        value: 100
    }, {
        label: "Indirizzo",
        value: 100
    }, {
        label: "Anno costruzione",
        value: 100
    }, {
        label: "Anno riqualificazione",
        value: null
    }
];


const CardDataEdificio = () => {
    // todo: implementare il retrieve nel backend

    return <Card className="@container/card col-span-3">
        <CardHeader>
            <h1 className="text-2xl font-bold text-primary tracking-tight">Dati Edificio</h1>
        </CardHeader>
        <CardContent>
            <div className="grid grid-cols-2 justify-start items-center gap-6">
                {
                    dataFake.map(item => {
                        return <Fragment key={ item.label }>
                            <div>
                                <p className="font-medium">{ item.label }</p>
                            </div>
                            <div className="flex items-center justify-center">
                                <p className="font-semibold">{ item.value ?? "Dato non disponibile" }</p>
                            </div>
                        </Fragment>;
                    })
                }
            </div>
        </CardContent>
    </Card>;
};

export default CardDataEdificio;