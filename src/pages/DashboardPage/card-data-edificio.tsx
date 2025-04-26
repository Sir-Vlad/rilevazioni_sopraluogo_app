import { Card, CardContent, CardHeader } from "@/components/ui/card.tsx";
import { Fragment, ReactNode }           from "react";
import { useEdifici }                    from "@/context/UseProvider.tsx";
import { CheckIcon, XIcon }              from "lucide-react";
import TitleCard                         from "@/components/title-card.tsx";
import { capitalize }                    from "@/helpers/helpers.ts";

const CardDataEdificio = () => {
    const {
              data,
              selectedEdificio
          } = useEdifici();

    const valueElement = (value: unknown) => {
        if (typeof value === "boolean") {
            return value ? <CheckIcon className="text-green-500" /> : <XIcon className="text-red-500" />;
        } else {
            const v: ReactNode = value as ReactNode ?? "Dato non disponibile";
            return <p className="font-semibold">{ v }</p>;
        }
    };

    return <Card className="@container/card col-span-3">
        <CardHeader>
            <TitleCard title="Dati Edificio" />
        </CardHeader>
        <CardContent>
            <div className="grid grid-cols-2 justify-start items-center gap-6">
                { data.length > 0 ? (data
                    .filter(value => value.chiave === selectedEdificio)
                    .map((value) => {
                        return Object.entries(value)
                                     .filter(([ key, _ ]) => key !== "note_riqualificazione")
                                     .map(([ key, value ]) => {
                                         return <Fragment key={ key }>
                                             <div>
                                                 <p className="font-medium">{ capitalize(key) }</p>
                                             </div>
                                             <div className="flex items-center justify-center">
                                                 { valueElement(value) }
                                             </div>
                                         </Fragment>;
                                     });

                    })) : (
                    <div className="col-span-2 h-34 flex items-center justify-center rounded-md border p-4">
                        <span>No results</span>
                    </div>)
                }
            </div>
        </CardContent>
    </Card>;
};

export default CardDataEdificio;